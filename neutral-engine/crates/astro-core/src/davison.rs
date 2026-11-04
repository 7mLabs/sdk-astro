//! Time/place relationship charts. Both variants recalculate an astronomical sky.
use super::*;
use serde_json::value::RawValue;

fn default_variant() -> String {
    "uncorrected".into()
}
const MC_TOLERANCE: f64 = 1e-6;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Request {
    #[serde(default)]
    include_metadata: bool,
    operation: String,
    person_a: Box<RawValue>,
    person_b: Box<RawValue>,
    #[serde(default = "default_variant")]
    variant: String,
    #[serde(default = "natal::default_house_system")]
    house_system: String,
    #[serde(default, deserialize_with = "natal::present")]
    domains: Option<Vec<String>>,
    #[serde(default, deserialize_with = "natal::present")]
    custom_profiles: Option<Vec<profiles::Profile>>,
    #[serde(default)]
    aspect_rules: natal::AspectSettings,
    #[serde(default, deserialize_with = "natal::present")]
    aspect_preset: Option<String>,
    #[serde(default, deserialize_with = "natal::present")]
    rulership: Option<String>,
}
fn mc(chart: &Value) -> f64 {
    chart["angles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["id"] == "midheaven")
        .unwrap()["longitude"]
        .as_f64()
        .unwrap()
}
fn average(a: f64, b: f64) -> f64 {
    a.min(b) + (a.max(b) - a.min(b)) / 2.
}
/// Same arithmetic coordinate means as Astrodienst's corrected/uncorrected DRC.
/// This deliberately is not a shortest-arc or a spherical geographical midpoint.
fn place(a: &Value, b: &Value) -> Value {
    json!({"latitude":average(a["latitude"].as_f64().unwrap(),b["latitude"].as_f64().unwrap()),"longitude":average(a["longitude"].as_f64().unwrap(),b["longitude"].as_f64().unwrap())})
}
fn at(
    jd: f64,
    location: &Value,
    system: &str,
    rules: &[AspectRule],
) -> Result<(Value, Value), String> {
    // Drop the session before compute_birth reacquires the Swiss provider lock.
    let u = astro_provider_swiss::EphemerisSession::new()?.utc_from_ut1(jd)?;
    let birth: natal::BirthInput = serde_json::from_value(json!({"utc":{"year":u.year,"month":u.month,"day":u.day,"hour":u.hour,"minute":u.minute,"second":u.second},"location":location,"houseSystem":system})).map_err(|e|e.to_string())?;
    let system = natal::validate_birth(&birth)?;
    natal::compute_birth(&birth, system, rules)
}
/// Follow the increasing MC continuously, away from the +/-180 signed-angle cut.
/// Search at most half a mean solar day in the direction of the shortest MC arc.
fn corrected(
    mean: f64,
    location: &Value,
    system: &str,
    rules: &[AspectRule],
    initial: &(Value, Value),
    target: f64,
) -> Result<((Value, Value), usize), String> {
    let delta = signed_difference(target, mc(&initial.0));
    if delta.abs() <= MC_TOLERANCE {
        return Ok((initial.clone(), 0));
    }
    let direction = delta.signum();
    let start = mc(&initial.0);
    let mut low = 0.;
    let mut high = 0.5;
    for iteration in 1..=48 {
        let offset = (low + high) / 2.;
        let sky = at(mean + direction * offset, location, system, rules)?;
        let travelled = if direction > 0. {
            normalize(mc(&sky.0) - start)
        } else {
            normalize(start - mc(&sky.0))
        };
        if signed_difference(target, mc(&sky.0)).abs() <= MC_TOLERANCE {
            return Ok((sky, iteration));
        }
        if travelled < delta.abs() {
            low = offset;
        } else {
            high = offset;
        }
    }
    Err("Corrected Davison MC search did not converge within 48 iterations".into())
}
fn calculate(r: Request) -> Result<Value, (&'static str, String)> {
    let invalid = |e: String| ("INVALID_INPUT", e);
    let failed = |e: String| ("CALCULATION_FAILED", e);
    if r.operation != "davison" || !["uncorrected", "corrected"].contains(&r.variant.as_str()) {
        return Err(invalid(
            "Expected operation davison and variant uncorrected or corrected".into(),
        ));
    }
    // Validate configuration before any provider work.
    if !["placidus", "wholeSign"].contains(&r.house_system.as_str()) {
        return Err(invalid("houseSystem must be placidus or wholeSign".into()));
    }
    let preset = r.aspect_preset.as_deref().unwrap_or("extended");
    if !["major", "extended"].contains(&preset)
        || (r.aspect_preset.is_some() && r.aspect_rules.provided)
    {
        return Err(invalid(
            "aspectPreset must be major or extended and cannot be combined with aspectRules".into(),
        ));
    }
    let rules = if r.aspect_rules.provided {
        r.aspect_rules.rules.clone()
    } else if preset == "extended" {
        domains::extended_rules()
    } else {
        default_rules()
    };
    validate_aspect_rules(&rules).map_err(invalid)?;
    let names = r
        .domains
        .unwrap_or_else(|| profiles::NAMES.into_iter().map(String::from).collect());
    if r.custom_profiles.as_ref().is_some_and(Vec::is_empty) {
        return Err(invalid(
            "customProfiles must contain 1 to 8 profiles when provided".into(),
        ));
    }
    let custom = r.custom_profiles.unwrap_or_default();
    let rulership = r.rulership.as_deref().unwrap_or("traditional");
    profiles::validate_selection(&names, &custom, rulership).map_err(invalid)?;
    let mut inputs = Vec::new();
    for (name, raw) in [("personA", r.person_a), ("personB", r.person_b)] {
        let birth: natal::BirthInput =
            serde_json::from_str(raw.get()).map_err(|e| invalid(format!("{name}: {e}")))?;
        let system = natal::validate_birth(&birth).map_err(|e| invalid(format!("{name}: {e}")))?;
        inputs.push((name, birth, system));
    }
    let mut skies = Vec::new();
    for (name, birth, system) in inputs {
        skies.push(
            natal::compute_birth(&birth, system, &rules)
                .map_err(|e| failed(format!("{name}: {e}")))?,
        );
    }
    let (a, ca) = &skies[0];
    let (b, cb) = &skies[1];
    let mean = average(
        ca["julianDayUt1"].as_f64().unwrap(),
        cb["julianDayUt1"].as_f64().unwrap(),
    );
    let location = place(&a["location"], &b["location"]);
    let initial = at(mean, &location, &r.house_system, &rules).map_err(failed)?;
    let (sky, iterations, target) = if r.variant == "corrected" {
        let delta = signed_difference(mc(b), mc(a));
        if (delta.abs() - 180.).abs() <= 1e-10 {
            return Err(failed(
                "Source MCs are antipodal; corrected Davison has no unique midpoint target".into(),
            ));
        }
        // Sorted source MCs ensure identical target rounding after swapping A/B.
        let lo = mc(a).min(mc(b));
        let hi = mc(a).max(mc(b));
        let target = normalize(lo + (hi - lo) / 2. + if hi - lo > 180. { 180. } else { 0. });
        let (sky, iterations) =
            corrected(mean, &location, &r.house_system, &rules, &initial, target)
                .map_err(failed)?;
        (sky, iterations, Some(target))
    } else {
        (initial, 0, None)
    };
    let (chart, mut calculation) = sky;
    let jd = calculation["julianDayUt1"].as_f64().unwrap();
    let provenance = json!({"variant":r.variant,"geographicMethod":"arithmeticCoordinateMean","timeMidpointScale":"UT1","meanJulianDayUt1":mean,"derivedJulianDayUt1":jd,"timeCorrectionSeconds":(jd-mean)*86400.,"targetMidheaven":target,"midheavenResidualDegrees":target.map(|t|signed_difference(t,mc(&chart))),"mcToleranceDegrees":MC_TOLERANCE,"iterations":iterations,"sourceCalculations":{"A":ca,"B":cb},"sourceBirths":{"A":{"utc":a["utc"],"location":a["location"]},"B":{"utc":b["utc"],"location":b["location"]}}});
    if r.include_metadata {
        calculation["operation"] = json!("davison");
    }
    calculation["scope"] = json!("davison-relationship-data");
    calculation["chartKind"] = json!("davisonRelationship");
    calculation["construction"] = json!("timePlaceMidpoint");
    calculation["variant"] = json!(r.variant);
    calculation["aspectPreset"] = json!(if r.aspect_rules.provided {
        "custom"
    } else {
        preset
    });
    calculation["rulership"] = json!(rulership);
    calculation["domainProfile"] = json!({"id":"sevenmlabs-domain-selection","version":"2.0"});
    calculation["reportProfile"] = json!({"id":"sevenmlabs-individual-report","version":"1.1"});
    calculation["customProfileCount"] = json!(custom.len());
    let mut data = domains::build_with_kind(
        chart,
        &rules,
        &names,
        rulership,
        &custom,
        "davisonRelationship",
    );
    data["subjectCount"] = json!(2);
    data["provenance"] = provenance;
    Ok(
        json!({"schemaVersion":"1.0","engineVersion":VERSION,"calculation":calculation,"data":data,"warnings":["Davison is a relationship chart derived from two births, not an actual birth or relationship event. Geography uses arithmetic latitude/longitude means, not a spherical or shortest-arc midpoint.","Moshier analytical ephemeris; UT1 midpoint and UTC conversion use Swiss built-in leap seconds and Delta T, not live Earth-orientation data."],"errors":[]}),
    )
}
pub(super) fn calculate_json(raw: &str) -> String {
    match serde_json::from_str::<Request>(raw) {
        Ok(r) => match calculate(r) {
            Ok(v) => v.to_string(),
            Err((code, e)) => error_json(code, &e),
        },
        Err(e) => error_json("INVALID_INPUT", &e.to_string()),
    }
}
