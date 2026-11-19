//! A question chart with explicitly versioned rules for significators and lunar aspects.
use super::*;

// The default identifies a specific edition of the rules. Callers omitting the
// selector keep these semantics when other rulesets are introduced.
#[derive(Default, Deserialize, Serialize)]
enum Ruleset {
    #[default]
    #[serde(rename = "western-traditional-v1")]
    WesternTraditionalV1,
}

impl Ruleset {
    fn calculate(&self, request: &HoraryRequest) -> Result<Value, (&'static str, String)> {
        match self {
            Self::WesternTraditionalV1 => western_traditional_v1(request),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HoraryRequest {
    operation: String,
    moment: natal::HoraryMoment,
    #[serde(deserialize_with = "numbers::i32")]
    quesited_house: i32,
    #[serde(default)]
    ruleset: Ruleset,
}

const CLASSICAL_BODIES: [&str; 7] = [
    "sun", "moon", "mercury", "venus", "mars", "jupiter", "saturn",
];

fn classical(body_id: &Value) -> bool {
    body_id
        .as_str()
        .is_some_and(|id| CLASSICAL_BODIES.contains(&id))
}

fn assign_rulers(chart: &mut Value) {
    for house in chart["houses"].as_array_mut().unwrap() {
        let longitude = house["longitude"].as_f64().unwrap();
        house["rulerBodyId"] = json!(domains::TRADITIONAL[(longitude / 30.).floor() as usize]);
    }
}

/// The same ruler may represent both parties, including when house 1 is selected.
fn significator(chart: &Value, house_number: usize) -> Value {
    let cusp = &chart["houses"][house_number - 1];
    json!({
        "houseNumber": house_number,
        "cuspLongitude": cusp["longitude"],
        "cuspSign": cusp["sign"],
        "rulerBodyId": cusp["rulerBodyId"],
    })
}

fn western_traditional_v1(request: &HoraryRequest) -> Result<Value, (&'static str, String)> {
    let invalid = |message| ("INVALID_INPUT", message);
    let failed = |message| ("CALCULATION_FAILED", message);
    if request.operation != "horary" || !(1..=12).contains(&request.quesited_house) {
        return Err(invalid("Expected horary and quesitedHouse in 1..12".into()));
    }
    let system = natal::validate_horary_moment(&request.moment).map_err(invalid)?;
    let rules: Vec<_> = [(0., 8.), (60., 6.), (90., 8.), (120., 8.), (180., 8.)]
        .into_iter()
        .map(|(angle, max_orb)| AspectRule { angle, max_orb })
        .collect();
    let (mut chart, mut calculation) =
        natal::horary_sky(&request.moment, system, &rules).map_err(failed)?;
    chart["placements"]
        .as_array_mut()
        .unwrap()
        .retain(|p| classical(&p["id"]));
    chart["aspects"]
        .as_array_mut()
        .unwrap()
        .retain(|a| classical(&a["body1"]) && classical(&a["body2"]));
    assign_rulers(&mut chart);
    let querent = significator(&chart, 1);
    let quesited = significator(&chart, request.quesited_house as usize);
    let jd = calculation["julianDayUt1"].as_f64().unwrap();
    let moon = events::horary_moon(jd, &rules).map_err(failed)?;
    calculation["operation"] = json!("horary");
    calculation["scope"] = json!("horary-indicators");
    calculation["chartKind"] = json!("horary");
    calculation["ruleset"] = json!(request.ruleset);
    calculation["rulesetVersion"] = json!(1);
    calculation["rulership"] = json!("traditional");
    calculation["bodyIds"] = json!(CLASSICAL_BODIES);
    Ok(json!({
        "schemaVersion": "1.0",
        "engineVersion": VERSION,
        "calculation": calculation,
        "data": {
            "chart": chart,
            "significators": {"querent":querent,"quesited":quesited,"moonBodyId":"moon"},
            "voidOfCourseMoon": moon,
        },
        "warnings": ["Swiss built-in leap seconds and Delta T model the relation between UTC and Earth rotation."],
        "errors": [],
    }))
}

pub(super) fn calculate_json(raw: &str) -> String {
    match serde_json::from_str::<HoraryRequest>(raw) {
        Ok(request) => match request.ruleset.calculate(&request) {
            Ok(value) => value.to_string(),
            Err((code, message)) => error_json(code, &message),
        },
        Err(error) => error_json("INVALID_INPUT", &error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> Value {
        json!({"operation":"horary","moment":{
            "utc":{"year":2000,"month":1,"day":1,"hour":12,"minute":0},
            "location":{"latitude":51.48,"longitude":0.0}
        }, "quesitedHouse": 7})
    }

    fn call(req: &Value) -> Value {
        serde_json::from_str(&super::super::calculate_json(&req.to_string())).unwrap()
    }

    #[test]
    fn default_ruleset_is_reproducible_and_scopes_all_chart_contacts() {
        let mut req = request();
        let response = call(&req);
        assert_eq!(response["errors"], json!([]));
        req["ruleset"] = json!("western-traditional-v1");
        req["moment"]["houseSystem"] = json!("regiomontanus");
        assert_eq!(call(&req), response);
        let metadata = &response["calculation"];
        assert_eq!(metadata["houseSystem"], "regiomontanus");
        assert_eq!(metadata["houseOrder"], "increasingLongitude");
        assert_eq!(metadata["rulership"], "traditional");
        assert_eq!(metadata["ruleset"], "western-traditional-v1");
        assert_eq!(metadata["rulesetVersion"], 1);
        assert_eq!(metadata["bodyIds"], json!(CLASSICAL_BODIES));
        assert_eq!(
            metadata["aspectRules"],
            json!([{"angle":0.0,"maxOrb":8.0},{"angle":60.0,"maxOrb":6.0},
                   {"angle":90.0,"maxOrb":8.0},{"angle":120.0,"maxOrb":8.0},
                   {"angle":180.0,"maxOrb":8.0}])
        );
        let data = &response["data"];
        let chart = &data["chart"];
        let bodies = chart["placements"].as_array().unwrap();
        assert_eq!(bodies.len(), 7);
        assert!(bodies.iter().all(|p| classical(&p["id"])));
        let aspects = chart["aspects"].as_array().unwrap();
        assert!(!aspects.is_empty());
        assert!(aspects
            .iter()
            .all(|a| classical(&a["body1"]) && classical(&a["body2"])));
        let roles = &data["significators"];
        assert_eq!(roles["querent"]["cuspLongitude"], chart["houseCusps"][0]);
        assert_eq!(roles["quesited"]["cuspLongitude"], chart["houseCusps"][6]);
        // Independent great-circle construction, Greenwich 2000-01-01 12:00 UTC.
        // On the equator mark points at RA = GMST + (house - 10) * 30°.
        // Intersect the plane of each mark and the north horizon point with
        // the ecliptic, selecting the longitude facing the mark. Using J2000
        // mean obliquity 23.4392911° and GMST 280.46061837° neglects nutation
        // and UT1-UTC; the 0.03° tolerance accounts for this approximation.
        let reference = [
            24.27826, 67.76370, 86.53891, 99.61424, 114.42381, 142.24517, 204.27826, 247.76370,
            266.53891, 279.61424, 294.42381, 322.24517,
        ];
        for (i, cusp) in reference.into_iter().enumerate() {
            assert!(
                signed_difference(chart["houseCusps"][i].as_f64().unwrap(), cusp).abs() < 0.03,
                "Regiomontanus H{}",
                i + 1
            );
        }
        assert_eq!(roles["querent"]["rulerBodyId"], "mars");
        assert_eq!(roles["quesited"]["rulerBodyId"], "venus");
        assert_eq!(roles["moonBodyId"], "moon");
        assert!(data["voidOfCourseMoon"]["isVoidOfCourse"].is_boolean());
    }

    fn polar_request(latitude: f64, longitude: f64, hour: i32) -> Value {
        json!({"operation":"horary","moment":{
            "utc":{"year":2026,"month":11,"day":19,"hour":hour,"minute":59,"second":59},
            "location":{"latitude":latitude,"longitude":longitude}
        },"quesitedHouse":7})
    }

    #[test]
    fn polar_regiomontanus_preserves_numbered_cusps_and_projected_planet_houses() {
        // References from direct Swiss 2.10.03 swe_houses_ex, and separately
        // swe_house_pos with each planet's longitude and ecliptic latitude 0.
        let north_cusps = [
            329.145236377,
            328.664712294,
            328.176758577,
            327.178158756,
            196.432434142,
            150.084405873,
            149.145236377,
            148.664712294,
            148.176758577,
            147.178158756,
            16.432434142,
            330.084405873,
        ];
        for (latitude, hour, rulers, houses) in [
            (78.2232, 16, ["saturn", "sun"], [4, 11, 4, 4, 9, 10, 11]),
            (-78.2232, 4, ["sun", "saturn"], [10, 5, 10, 11, 3, 4, 5]),
        ] {
            let response = call(&polar_request(latitude, 15.6469, hour));
            assert_eq!(response["errors"], json!([]));
            assert_eq!(response["calculation"]["houseOrder"], "decreasingLongitude");
            let chart = &response["data"]["chart"];
            if latitude > 0. {
                for (i, cusp) in north_cusps.iter().enumerate() {
                    assert!((chart["houseCusps"][i].as_f64().unwrap() - cusp).abs() < 1e-5);
                    assert_eq!(chart["houseCusps"][i], chart["houses"][i]["longitude"]);
                    assert_eq!(chart["houses"][i]["number"], i + 1);
                }
            }
            let roles = &response["data"]["significators"];
            assert_eq!(roles["querent"]["rulerBodyId"], rulers[0]);
            assert_eq!(roles["quesited"]["rulerBodyId"], rulers[1]);
            assert_eq!(roles["querent"]["cuspLongitude"], chart["houseCusps"][0]);
            assert_eq!(roles["quesited"]["cuspLongitude"], chart["houseCusps"][6]);
            for (i, house) in houses.into_iter().enumerate() {
                assert_eq!(
                    chart["placements"][i]["house"], house,
                    "{}",
                    CLASSICAL_BODIES[i]
                );
            }
        }
    }

    #[test]
    fn polar_regiomontanus_requires_distinct_cusps_and_a_defined_location() {
        for (latitude, longitude) in [(78.2232, 17.51775540282449), (-78.2232, 75.0083109542204)] {
            let response = call(&polar_request(latitude, longitude, 16));
            assert_eq!(response["errors"][0]["code"], "CALCULATION_FAILED");
            assert!(response["errors"][0]["message"]
                .as_str()
                .unwrap()
                .contains("12 distinct houses"));
            for offset in [-1e-6, 1e-6] {
                assert_eq!(
                    call(&polar_request(latitude, longitude + offset, 16))["errors"],
                    json!([])
                );
            }
        }
        for latitude in [89.999999999, -89.999999999] {
            assert_eq!(
                call(&polar_request(latitude, 15.6469, 16))["errors"],
                json!([])
            );
        }
        for latitude in [-90., 90.] {
            assert_eq!(
                call(&polar_request(latitude, 15.6469, 16))["errors"][0]["code"],
                "INVALID_INPUT"
            );
        }
    }

    #[test]
    fn house_selection_uses_traditional_rulers_and_allows_a_shared_significator() {
        let houses: Vec<_> = SIGNS
            .iter()
            .enumerate()
            .map(|(i, sign)| json!({"longitude":30.0 * i as f64, "sign":sign}))
            .collect();
        let mut chart = json!({"houses":houses});
        assign_rulers(&mut chart);
        for (house, expected) in [(8, "mars"), (11, "saturn"), (12, "jupiter")] {
            assert_eq!(significator(&chart, house)["rulerBodyId"], expected);
        }
        let mut req = request();
        req["quesitedHouse"] = json!(1);
        req["moment"]["houseSystem"] = json!("wholeSign");
        let response = call(&req);
        assert_eq!(response["errors"], json!([]));
        let roles = &response["data"]["significators"];
        assert_eq!(roles["querent"], roles["quesited"]);
        assert_eq!(response["calculation"]["houseSystem"], "wholeSign");
        assert_eq!(response["calculation"]["houseOrder"], "increasingLongitude");
    }

    #[test]
    fn errors_reject_unknown_methods_and_invalid_question_context() {
        for (field, value) in [
            ("ruleset", json!("lilly")),
            ("ruleset", Value::Null),
            ("quesitedHouse", json!(0)),
            ("quesitedHouse", json!(13)),
            ("quesitedHouse", json!(2.5)),
            ("quesitedHouse", json!("7")),
            ("question", json!("free-form")),
        ] {
            let mut req = request();
            req[field] = value;
            let response = call(&req);
            assert_eq!(response["errors"][0]["code"], "INVALID_INPUT", "{req}");
            assert!(response["data"].is_null());
        }
        let mut req = request();
        req["moment"]["houseSystem"] = json!("unspecified");
        assert_eq!(call(&req)["errors"][0]["code"], "INVALID_INPUT");
        let mut natal = request()["moment"].clone();
        natal["operation"] = json!("natal");
        natal["houseSystem"] = json!("regiomontanus");
        assert_eq!(call(&natal)["errors"][0]["code"], "INVALID_INPUT");
    }
}
