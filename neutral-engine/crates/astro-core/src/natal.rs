use super::*;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Utc {
    #[serde(deserialize_with = "numbers::i32")]
    year: i32,
    #[serde(deserialize_with = "numbers::i32")]
    month: i32,
    #[serde(deserialize_with = "numbers::i32")]
    day: i32,
    #[serde(deserialize_with = "numbers::i32")]
    hour: i32,
    #[serde(deserialize_with = "numbers::i32")]
    minute: i32,
    #[serde(default)]
    second: f64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct HoraryMoment {
    utc: Utc,
    location: Location,
    #[serde(default = "horary_house_system")]
    house_system: String,
}

fn horary_house_system() -> String {
    "regiomontanus".into()
}

pub(super) fn validate_horary_moment(moment: &HoraryMoment) -> Result<u8, String> {
    validate_time_and_location(&moment.utc, &moment.location)?;
    match moment.house_system.as_str() {
        "regiomontanus" => Ok(b'R'),
        "placidus" => Ok(b'P'),
        "wholeSign" => Ok(b'W'),
        _ => Err("Horary houseSystem must be regiomontanus, placidus or wholeSign".into()),
    }
}

pub(super) fn horary_sky(
    moment: &HoraryMoment,
    system: u8,
    rules: &[AspectRule],
) -> Result<(Value, Value), String> {
    let (chart, mut calculation, order) = compute_sky(
        &moment.utc,
        &moment.location,
        &moment.house_system,
        system,
        rules,
    )?;
    calculation["houseOrder"] = json!(order);
    Ok((chart, calculation))
}

pub(super) fn default_house_system() -> String {
    "placidus".into()
}

#[derive(Default)]
pub(super) struct AspectSettings {
    pub provided: bool,
    pub rules: Vec<AspectRule>,
}
impl<'de> Deserialize<'de> for AspectSettings {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self {
            provided: true,
            rules: Vec::<AspectRule>::deserialize(d)?,
        })
    }
}
pub(super) fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct BirthInput {
    utc: Utc,
    location: Location,
    #[serde(default = "default_house_system")]
    house_system: String,
}

/// Explicit opt-in: legacy calls retain the ten-body population and byte contract.
#[derive(Deserialize)]
#[serde(untagged)]
enum AdditionalPoints {
    Preset(String),
    Ids(Vec<String>),
}
impl AdditionalPoints {
    fn ids(&self) -> Result<Vec<String>, String> {
        let ids = match self {
            Self::Preset(p) if p == "standard" => vec!["chiron", "trueNorthNode", "meanLilith"]
                .into_iter()
                .map(String::from)
                .collect(),
            Self::Preset(p) if p == "all" => vec![
                "chiron",
                "trueNorthNode",
                "trueSouthNode",
                "meanNorthNode",
                "meanSouthNode",
                "meanLilith",
                "osculatingLilith",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            Self::Preset(_) => return Err("additionalPoints preset must be standard or all".into()),
            Self::Ids(ids) => ids.clone(),
        };
        let mut seen = HashSet::new();
        if ids.len() > 7
            || ids.iter().any(|id| {
                ![
                    "chiron",
                    "trueNorthNode",
                    "trueSouthNode",
                    "meanNorthNode",
                    "meanSouthNode",
                    "meanLilith",
                    "osculatingLilith",
                ]
                .contains(&id.as_str())
                    || !seen.insert(id)
            })
        {
            return Err("additionalPoints contains an unknown or duplicate ID".into());
        }
        Ok(ids)
    }
}
fn extend_natal(
    natal: &mut Value,
    calculation: &mut Value,
    ids: &[String],
    rules: &[AspectRule],
) -> Result<(), String> {
    if ids.is_empty() {
        return Ok(());
    }
    let requested: Vec<_> = ids
        .iter()
        .map(|id| {
            (
                id.as_str(),
                match id.as_str() {
                    "chiron" => 15,
                    "trueNorthNode" | "trueSouthNode" => 11,
                    "meanNorthNode" | "meanSouthNode" => 10,
                    "meanLilith" => 12,
                    _ => 13,
                },
            )
        })
        .collect();
    let coordinates = astro_provider_swiss::additional_points(
        calculation["julianDayUt1"].as_f64().unwrap(),
        &requested,
    )?;
    let positions: Vec<_> = ids
        .iter()
        .zip(&coordinates)
        .map(|(id, p)| Position {
            id: id.clone(),
            longitude: normalize(p[0] + if id.ends_with("SouthNode") { 180. } else { 0. }),
            speed: Some(p[3]),
        })
        .collect();
    let houses: Vec<_> = natal["houseCusps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_f64().unwrap())
        .collect();
    let mut extra = serde_json::to_value(placements(&positions, Some(&houses))).unwrap();
    for ((point, id), p) in extra
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .zip(ids)
        .zip(&coordinates)
    {
        point["latitude"] = json!(if id.ends_with("SouthNode") {
            -p[1]
        } else {
            p[1]
        });
        point["distanceAu"] = json!(p[2]);
        point["kind"] = json!(if id == "chiron" {
            "minorBody"
        } else if id.contains("Node") {
            "lunarNode"
        } else {
            "lunarApogee"
        });
        point["definition"] = json!(id);
        point["ephemeris"] = json!(if id == "chiron" {
            "swiss-asteroid-seas_18-with-moshier-reference"
        } else {
            "moshier"
        });
    }
    let mut all: Vec<Position> = natal["placements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| Position {
            id: p["id"].as_str().unwrap().into(),
            longitude: p["longitude"].as_f64().unwrap(),
            speed: p["speed"].as_f64(),
        })
        .collect();
    all.extend(positions);
    natal["additionalPoints"] = extra;
    natal["aspects"] = json!(aspects(&all, None, rules));
    calculation["additionalPoints"] = json!({"ids":ids,"definitions":"explicit mean/true node and mean/osculating lunar apogee; South Node is the antipode of its matching North Node","dataFile":if ids.iter().any(|id| id == "chiron") { json!({"name":"seas_18.se1","sha256":"a2cd8fc33807c78ca9a700c91c2e042258b12fc4796519e00781440b5ad8b2e2","delivery":"embedded"}) } else { Value::Null }});
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct NatalRequest {
    #[serde(default)]
    include_metadata: bool,
    #[serde(default, deserialize_with = "present")]
    additional_points: Option<AdditionalPoints>,
    operation: String,
    utc: Utc,
    location: Location,
    #[serde(default = "default_house_system")]
    house_system: String,
    #[serde(default)]
    aspect_rules: AspectSettings,
    #[serde(default, deserialize_with = "present")]
    domains: Option<Vec<String>>,
    #[serde(default, deserialize_with = "present")]
    rulership: Option<String>,
    #[serde(default, deserialize_with = "present")]
    aspect_preset: Option<String>,
    #[serde(default, deserialize_with = "present")]
    custom_profiles: Option<Vec<profiles::Profile>>,
}

fn validate_natal(r: &NatalRequest, rules: &[AspectRule]) -> Result<u8, String> {
    if r.operation != "natal" && r.operation != "natalDomains" {
        return Err("Expected natal or natalDomains operation".into());
    }
    if r.operation == "natal"
        && (r.domains.is_some() || r.rulership.is_some() || r.custom_profiles.is_some())
    {
        return Err("domains, rulership and customProfiles require operation natalDomains".into());
    }
    let system = validate_birth_parts(&r.utc, &r.location, &r.house_system)?;
    validate_aspect_rules(rules)?;
    Ok(system)
}

pub(super) fn validate_birth(birth: &BirthInput) -> Result<u8, String> {
    validate_birth_parts(&birth.utc, &birth.location, &birth.house_system)
}

fn validate_birth_parts(utc: &Utc, location: &Location, house_system: &str) -> Result<u8, String> {
    validate_time_and_location(utc, location)?;
    match house_system {
        "placidus" => Ok(b'P'),
        "wholeSign" => Ok(b'W'),
        _ => Err("houseSystem must be placidus or wholeSign".into()),
    }
}

fn validate_time_and_location(utc: &Utc, location: &Location) -> Result<(), String> {
    let u = utc;
    if !(1800..=2399).contains(&u.year) || !(1..=12).contains(&u.month) {
        return Err("UTC requires a Gregorian year 1800..2399 and month 1..12".into());
    }
    let leap = u.year % 4 == 0 && (u.year % 100 != 0 || u.year % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if !(1..=days[(u.month - 1) as usize]).contains(&u.day)
        || !(0..=23).contains(&u.hour)
        || !(0..=59).contains(&u.minute)
        || !u.second.is_finite()
        || !(0.0..60.0).contains(&u.second)
    {
        return Err("UTC date/time is invalid; leap-second input is not supported".into());
    }
    if !location.latitude.is_finite()
        || !location.longitude.is_finite()
        || !(-90.0..=90.0).contains(&location.latitude)
        || !(-180.0..=180.0).contains(&location.longitude)
    {
        return Err("Latitude must be -90..90 and longitude -180..180, in degrees".into());
    }
    if location.latitude.abs() == 90.0 {
        return Err("House angles are undefined at the geographic poles".into());
    }
    Ok(())
}

pub(super) fn compute_birth(
    birth: &BirthInput,
    system: u8,
    rules: &[AspectRule],
) -> Result<(Value, Value), String> {
    let (chart, calculation, _) = compute_sky(
        &birth.utc,
        &birth.location,
        &birth.house_system,
        system,
        rules,
    )?;
    Ok((chart, calculation))
}

// In a polar Regiomontanus chart Swiss can return the numbered house cusps
// in decreasing longitude. Keep the original house numbers in either direction.
fn sky_placements(
    positions: &[Position],
    cusps: &[f64],
    system: u8,
) -> Result<(Vec<Placement>, &'static str), String> {
    if system != b'R' {
        validate_houses(&Some(cusps.to_vec()))?;
        return Ok((placements(positions, Some(cusps)), "increasingLongitude"));
    }
    let invalid = "Regiomontanus cusps do not define 12 distinct houses at this time and location";
    if cusps.len() != 12 || !cusps.iter().all(|cusp| (0.0..360.0).contains(cusp)) {
        return Err(invalid.into());
    }
    // When the common north/south horizon axis of the house circles lies on
    // the ecliptic, all cusps collapse onto its two ends. Fused and non-fused
    // Swiss sidereal-time arithmetic can spread this singular result by a
    // few 1e-8 degrees. Test the entire axis, never an individual house width.
    const AXIS_TOLERANCE_DEG: f64 = 1e-7;
    if cusps.iter().all(|cusp| {
        let offset = (cusp - cusps[0]).rem_euclid(180.);
        offset.min(180. - offset) <= AXIS_TOLERANCE_DEG
    }) {
        return Err(invalid.into());
    }
    let edges = || {
        cusps
            .iter()
            .copied()
            .zip(cusps.iter().copied().cycle().skip(1))
    };
    // A strict cycle has one wrap at 0°. Compare raw coordinates because
    // legitimate cusp separations can be smaller than 1e-9° near a pole.
    let increases = edges().filter(|(start, end)| start < end).count();
    let decreases = edges().filter(|(start, end)| start > end).count();
    let (direction, order) = match (increases, decreases) {
        (11, 1) => (1.0, "increasingLongitude"),
        (1, 11) => (-1.0, "decreasingLongitude"),
        _ => return Err(invalid.into()),
    };
    let mut bodies = placements(positions, None);
    for body in &mut bodies {
        let longitude = body.longitude * direction;
        body.house = edges()
            .position(|(start, end)| {
                let start = start * direction;
                let end = end * direction;
                if start < end {
                    (start..end).contains(&longitude)
                } else {
                    longitude >= start || longitude < end
                }
            })
            .map(|index| index + 1);
    }
    Ok((bodies, order))
}

fn compute_sky(
    u: &Utc,
    location: &Location,
    house_system: &str,
    system: u8,
    rules: &[AspectRule],
) -> Result<(Value, Value, &'static str), String> {
    let sky = astro_provider_swiss::natal(
        [u.year, u.month, u.day, u.hour, u.minute],
        u.second,
        location.latitude,
        location.longitude,
        system,
    )?;
    let ids = [
        "sun", "moon", "mercury", "venus", "mars", "jupiter", "saturn", "uranus", "neptune",
        "pluto",
    ];
    let positions: Vec<Position> = sky
        .bodies
        .iter()
        .zip(ids)
        .map(|(p, id)| Position {
            id: id.into(),
            longitude: p[0],
            speed: Some(p[3]),
        })
        .collect();
    let (planet_placements, house_order) = sky_placements(&positions, &sky.houses, system)?;
    let mut bodies = serde_json::to_value(planet_placements).unwrap();
    for (body, coords) in bodies.as_array_mut().unwrap().iter_mut().zip(&sky.bodies) {
        body["latitude"] = json!(coords[1]);
        body["distanceAu"] = json!(coords[2]);
    }
    let angles: Vec<Position> = [
        ("ascendant", sky.ascendant),
        ("midheaven", sky.midheaven),
        ("descendant", normalize(sky.ascendant + 180.)),
        ("imumCoeli", normalize(sky.midheaven + 180.)),
    ]
    .into_iter()
    .map(|(id, longitude)| Position {
        id: id.into(),
        longitude,
        speed: None,
    })
    .collect();
    let houses: Vec<Value> = sky.houses.iter().enumerate().map(|(i, cusp)| {
        let longitude = normalize(*cusp);
        json!({"number": i+1, "longitude": longitude, "sign": SIGNS[(longitude / 30.).floor() as usize], "degreeInSign": longitude % 30.})
    }).collect();
    let natal = json!({"utc":u, "location":location, "placements":bodies,
        "angles":placements(&angles, None), "houses":houses, "houseCusps":sky.houses,
        "aspects":aspects(&positions, None, rules)});
    let calculation = json!({"scope":"basic-natal", "provider":"swiss-ephemeris", "providerVersion":"2.10.03",
            "ephemeris":"moshier", "zodiac":"tropical", "coordinates":"geocentric",
            "positionType":"apparent", "referenceFrame":"ecliptic-of-date", "calendar":"gregorian",
            "inputTimeScale":"UTC", "houseTimeScale":"UT1", "planetTimeScale":"TT",
            "timeModel":"Swiss built-in leap seconds and Delta T; pre-1972 civil input treated as UT1",
            "julianDayTt":sky.jd_tt, "julianDayUt1":sky.jd_ut1, "houseSystem":house_system,
            "angleUnit":"degrees", "speedUnit":"degrees/day", "distanceUnit":"AU",
            "aspectRules":rules.iter().map(|v| json!({"angle":v.angle,"maxOrb":v.max_orb})).collect::<Vec<_>>()});
    Ok((natal, calculation, house_order))
}

pub(super) fn calculate_json(raw: &str) -> String {
    let r: NatalRequest = match serde_json::from_str(raw) {
        Ok(r) => r,
        Err(e) => return error_json("INVALID_INPUT", &e.to_string()),
    };
    let ids = match r
        .additional_points
        .as_ref()
        .map(AdditionalPoints::ids)
        .transpose()
    {
        Ok(ids) => ids.unwrap_or_default(),
        Err(e) => return error_json("INVALID_INPUT", &e),
    };
    let is_domains = r.operation == "natalDomains";
    let preset =
        r.aspect_preset
            .as_deref()
            .unwrap_or(if is_domains { "extended" } else { "major" });
    if !["major", "extended"].contains(&preset)
        || (r.aspect_preset.is_some() && r.aspect_rules.provided)
    {
        return error_json(
            "INVALID_INPUT",
            "aspectPreset must be major or extended and cannot be combined with aspectRules",
        );
    }
    let rules = if r.aspect_rules.provided {
        r.aspect_rules.rules.clone()
    } else if preset == "extended" {
        domains::extended_rules()
    } else {
        default_rules()
    };
    let names = r
        .domains
        .clone()
        .unwrap_or_else(|| profiles::NAMES.into_iter().map(String::from).collect());
    let rulership = r.rulership.as_deref().unwrap_or("traditional");
    let custom = r.custom_profiles.as_deref().unwrap_or(&[]);
    if is_domains {
        if r.custom_profiles.is_some() && custom.is_empty() {
            return error_json(
                "INVALID_INPUT",
                "customProfiles must contain 1 to 8 profiles when provided",
            );
        }
        if let Err(e) = profiles::validate_selection(&names, custom, rulership) {
            return error_json("INVALID_INPUT", &e);
        }
    }
    let system = match validate_natal(&r, &rules) {
        Ok(s) => s,
        Err(e) => return error_json("INVALID_INPUT", &e),
    };
    let birth = BirthInput {
        utc: r.utc,
        location: r.location,
        house_system: r.house_system,
    };
    let (mut natal, mut calculation) = match compute_birth(&birth, system, &rules) {
        Ok(values) => values,
        Err(e) => return error_json("CALCULATION_FAILED", &e),
    };
    if let Err(e) = extend_natal(&mut natal, &mut calculation, &ids, &rules) {
        return error_json("CALCULATION_FAILED", &e);
    }
    calculation["scope"] = json!(if is_domains {
        "natal-domain-data"
    } else {
        "basic-natal"
    });
    let data = if is_domains {
        domains::build(natal, &rules, &names, rulership, custom)
    } else {
        natal
    };
    if r.include_metadata {
        calculation["operation"] = json!(if is_domains { "natalDomains" } else { "natal" });
        calculation["chartKind"] = json!("individualNatal");
    }
    let mut result = json!({"schemaVersion":"1.0", "engineVersion":VERSION,
        "calculation":calculation,
        "data":data,
        "warnings":["Moshier analytical ephemeris; no JPL/Swiss data files. Future UTC uses the provider's built-in time model, not live Earth-orientation data."],
        "errors":[]});
    if ids.iter().any(|id| id == "chiron") {
        result["warnings"][0] = json!("Sun–Pluto use Moshier. Chiron uses the embedded Swiss asteroid series; additional point definitions and file provenance are in calculation.additionalPoints. Future UTC uses built-in time tables, not live Earth-orientation data.");
    }
    if is_domains || r.aspect_preset.is_some() {
        result["calculation"]["aspectPreset"] = json!(if r.aspect_rules.provided {
            "custom"
        } else {
            preset
        });
    }
    if is_domains {
        result["calculation"]["rulership"] = json!(rulership);
        result["calculation"]["domainProfile"] =
            json!({"id":"sevenmlabs-domain-selection","version":"2.0"});
        result["calculation"]["chartKind"] = json!("individualNatal");
        result["calculation"]["reportProfile"] =
            json!({"id":"sevenmlabs-individual-report","version":"1.1"});
        result["calculation"]["customProfileCount"] = json!(custom.len());
        result["warnings"].as_array_mut().unwrap().push(json!("Domain groups follow the declared selection profile; no scores or predictions are produced."));
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> Value {
        json!({"operation":"natal", "utc":{"year":2000,"month":1,"day":1,"hour":12,"minute":0},
        "location":{"latitude":10.8231,"longitude":106.6297}})
    }
    fn run(r: Value) -> Value {
        serde_json::from_str(&super::super::calculate_json(&r.to_string())).unwrap()
    }

    #[test]
    fn horary_reverse_cusps_include_their_start_across_the_zodiac_seam() {
        let cusps: Vec<_> = (0..12).map(|i| normalize(-30. * i as f64)).collect();
        let positions: Vec<_> = [0., 359., 330., 329.9, 30., 0.001]
            .into_iter()
            .map(|longitude| Position {
                id: "test".into(),
                longitude,
                speed: None,
            })
            .collect();
        let (bodies, order) = sky_placements(&positions, &cusps, b'R').unwrap();
        assert_eq!(order, "decreasingLongitude");
        for (p, expected) in bodies.iter().zip([1, 1, 2, 2, 12, 12]) {
            assert_eq!(p.house, Some(expected));
        }
        assert!(sky_placements(&positions, &cusps, b'P').is_err());
        let mut disordered = cusps.clone();
        disordered.swap(3, 4);
        assert!(sky_placements(&positions, &disordered, b'R').is_err());
        let mut duplicated = cusps;
        duplicated[1] = duplicated[0];
        assert!(sky_placements(&positions, &duplicated, b'R').is_err());
    }

    #[test]
    fn horary_rejects_numerically_split_antipodal_cusps() {
        // Swiss 2.10.03 with FMA disabled, 2026-11-19T16:59:59Z,
        // 78.2232° N, 17.51775540282449° E. All twelve outputs are distinct
        // and ordered even though the house circles intersect on one axis.
        let cusps = [
            149.12741870184715,
            149.12741870208114,
            149.12741870232114,
            149.12741870282008,
            329.12741868334246,
            329.1274187013962,
            329.12741870184715,
            329.1274187020812,
            329.12741870232117,
            329.1274187028201,
            149.12741868334246,
            149.12741870139624,
        ];
        assert!(sky_placements(&[], &cusps, b'R').is_err());

        // A second distinct axis restores the partition, including the very
        // narrow houses elsewhere in the ring.
        let mut resolved = cusps;
        resolved[3] += 5e-6;
        resolved[9] += 5e-6;
        assert!(sky_placements(&[], &resolved, b'R').is_ok());
    }
    #[test]
    fn natal_has_real_sky_and_houses() {
        let v = run(request());
        assert_eq!(v["errors"], json!([]));
        assert!(v["calculation"].get("houseOrder").is_none());
        assert_eq!(v["data"]["placements"].as_array().unwrap().len(), 10);
        assert_eq!(v["data"]["houses"].as_array().unwrap().len(), 12);
        let sun = v["data"]["placements"][0]["longitude"].as_f64().unwrap();
        assert!((sun - 280.369).abs() < 0.002);
        assert_eq!(v["data"]["placements"][0]["sign"], "Capricorn");
        assert!(v["data"]["placements"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["house"].is_number()));
    }
    #[test]
    fn validates_calendar_and_options() {
        for (key, value) in [
            ("month", 13),
            ("day", 32),
            ("hour", 24),
            ("minute", 60),
            ("second", 60),
            ("year", 2400),
        ] {
            let mut r = request();
            r["utc"][key] = json!(value);
            assert!(!run(r)["errors"].as_array().unwrap().is_empty());
        }
        let mut r = request();
        r["utc"]["year"] = json!(1900);
        r["utc"]["month"] = json!(2);
        r["utc"]["day"] = json!(29);
        assert!(!run(r)["errors"].as_array().unwrap().is_empty());
        let mut r = request();
        r["utc"]["month"] = json!(2);
        r["utc"]["day"] = json!(29);
        assert_eq!(run(r)["errors"], json!([]));
    }
    #[test]
    fn integer_utc_notations_return_the_same_chart() {
        let expected = run(request());
        let float_request = r#"{"operation":"natal","utc":{"year":2000.0,"month":1.0,"day":1e0,"hour":1.2e1,"minute":-0.0},"location":{"latitude":10.8231,"longitude":106.6297}}"#;
        let actual: Value =
            serde_json::from_str(&super::super::calculate_json(float_request)).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(
            actual["data"]["utc"].to_string(),
            r#"{"day":1,"hour":12,"minute":0,"month":1,"second":0.0,"year":2000}"#
        );
        for key in ["year", "month", "day", "hour", "minute"] {
            for invalid in [json!(1.5), json!("1"), json!(true), Value::Null] {
                let mut r = request();
                r["utc"][key] = invalid;
                assert_eq!(run(r)["errors"][0]["code"], "INVALID_INPUT");
            }
        }
    }
    #[test]
    fn zero_coordinates_time_sensitivity_and_polar_error() {
        let mut r = request();
        r["location"] = json!({"latitude":0,"longitude":0});
        let a = run(r.clone());
        assert_eq!(a["errors"], json!([]));
        r["utc"]["hour"] = json!(13);
        let b = run(r.clone());
        assert_ne!(
            a["data"]["placements"][1]["longitude"],
            b["data"]["placements"][1]["longitude"]
        );
        assert_ne!(
            a["data"]["angles"][0]["longitude"],
            b["data"]["angles"][0]["longitude"]
        );
        r["location"]["latitude"] = json!(80);
        assert_eq!(run(r.clone())["errors"][0]["code"], "CALCULATION_FAILED");
        r["houseSystem"] = json!("wholeSign");
        assert_eq!(run(r)["errors"], json!([]));
    }
    #[test]
    fn concurrent_options_are_isolated() {
        let a = request();
        let mut b = request();
        b["houseSystem"] = json!("wholeSign");
        b["location"]["latitude"] = json!(-33.86);
        let expected_a = run(a.clone());
        let expected_b = run(b.clone());
        std::thread::scope(|s| {
            for i in 0..8 {
                let (r, e) = if i % 2 == 0 {
                    (&a, &expected_a)
                } else {
                    (&b, &expected_b)
                };
                s.spawn(move || {
                    for _ in 0..25 {
                        assert_eq!(&run(r.clone()), e);
                    }
                });
            }
        });
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    fn request(year: i32) -> Value {
        json!({"operation":"natal","utc":{"year":year,"month":2,"day":7,"hour":23,"minute":0},"location":{"latitude":10.8231,"longitude":106.6297}})
    }
    #[test]
    fn extended_points_are_opt_in_finite_and_preserve_original_bodies() {
        for year in [1800, 1900, 1997, 2000, 2100, 2399] {
            let mut r = request(year);
            let base: Value = serde_json::from_str(&calculate_json(&r.to_string())).unwrap();
            assert!(base["data"]["additionalPoints"].is_null());
            r["additionalPoints"] = json!("all");
            let extra: Value = serde_json::from_str(&calculate_json(&r.to_string())).unwrap();
            assert_eq!(extra["errors"], json!([]), "{year}: {extra}");
            assert_eq!(base["data"]["placements"], extra["data"]["placements"]);
            assert!(extra["calculation"]["domainProfile"].is_null());
            let points = extra["data"]["additionalPoints"].as_array().unwrap();
            assert_eq!(points.len(), 7);
            for p in points {
                assert!((0.0..360.).contains(&p["longitude"].as_f64().unwrap()));
                assert!(p["speed"].as_f64().unwrap().is_finite());
            }
            for (north, south) in [
                ("trueNorthNode", "trueSouthNode"),
                ("meanNorthNode", "meanSouthNode"),
            ] {
                let a = points.iter().find(|p| p["id"] == north).unwrap();
                let b = points.iter().find(|p| p["id"] == south).unwrap();
                assert!(
                    (signed_difference(
                        a["longitude"].as_f64().unwrap(),
                        b["longitude"].as_f64().unwrap()
                    )
                    .abs()
                        - 180.)
                        .abs()
                        < 1e-10
                );
                assert_eq!(a["speed"], b["speed"]);
            }
        }
    }
    #[test]
    fn natal_aspect_preset_is_not_domain_metadata() {
        let mut r = request(2000);
        r["aspectPreset"] = json!("extended");
        let out: Value = serde_json::from_str(&calculate_json(&r.to_string())).unwrap();
        assert_eq!(out["errors"], json!([]));
        assert_eq!(out["calculation"]["aspectPreset"], "extended");
        assert!(out["calculation"]["domainProfile"].is_null());
        assert_eq!(out["warnings"].as_array().unwrap().len(), 1);
    }
    #[test]
    fn definitions_validation_and_domain_population() {
        let mut r = request(1997);
        r["additionalPoints"] = json!("standard");
        r["operation"] = json!("natalDomains");
        let result: Value = serde_json::from_str(&calculate_json(&r.to_string())).unwrap();
        assert_eq!(result["errors"], json!([]), "{result}");
        assert_eq!(
            result["data"]["natal"]["additionalPoints"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
        assert_eq!(
            result["data"]["natal"]["placements"]
                .as_array()
                .unwrap()
                .len(),
            10
        );
        assert!(result["data"]["context"]["points"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"] == "chiron" && p["kind"] == "minorBody"));
        for v in [
            json!("wrong"),
            json!(["northNode"]),
            json!(["chiron", "chiron"]),
            Value::Null,
        ] {
            r["additionalPoints"] = v;
            let out: Value = serde_json::from_str(&calculate_json(&r.to_string())).unwrap();
            assert!(!out["errors"].as_array().unwrap().is_empty());
        }
    }
}
