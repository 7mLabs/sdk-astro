use astro_core::{calculate_json, signed_difference};
use serde_json::{json, Value};
fn request() -> Value {
    let mut r: Value =
        serde_json::from_str(include_str!("../../../../examples/composite-request.json")).unwrap();
    for key in ["houseMethod", "antipodalPolicy"] {
        r.as_object_mut().unwrap().remove(key);
    }
    r["operation"] = json!("davison");
    r
}
fn run(r: &Value) -> Value {
    serde_json::from_str(&calculate_json(&r.to_string())).unwrap()
}
fn success(r: &Value) -> Value {
    let v = run(r);
    assert_eq!(v["errors"], json!([]), "{v}");
    v
}
fn mc(v: &Value) -> f64 {
    v["angles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "midheaven")
        .unwrap()["longitude"]
        .as_f64()
        .unwrap()
}
#[test]
fn physical_sky_reports_and_corrected_target() {
    for variant in ["uncorrected", "corrected"] {
        let mut r = request();
        r["variant"] = json!(variant);
        let v = success(&r);
        let d = &v["data"];
        let p = &d["provenance"];
        assert_eq!(d["chartKind"], "davisonRelationship");
        assert_eq!(d["subjectCount"], 2);
        assert!(
            (d["natal"]["location"]["latitude"].as_f64().unwrap() - (10.8231 + 21.0278) / 2.).abs()
                < 1e-12
        );
        let mean = (p["sourceCalculations"]["A"]["julianDayUt1"]
            .as_f64()
            .unwrap()
            + p["sourceCalculations"]["B"]["julianDayUt1"]
                .as_f64()
                .unwrap())
            / 2.;
        assert_eq!(p["meanJulianDayUt1"], mean);
        let direct = success(
            &json!({"operation":"natal","utc":d["natal"]["utc"],"location":d["natal"]["location"],"aspectPreset":"extended"}),
        );
        assert_eq!(direct["data"], d["natal"]);
        assert_eq!(d["context"]["points"].as_array().unwrap().len(), 26);
        assert_eq!(d["context"]["relations"].as_array().unwrap().len(), 325);
        assert_eq!(d["domains"].as_object().unwrap().len(), 10);
        for view in d["domains"].as_object().unwrap().values() {
            assert_eq!(view["report"]["chartKind"], "davisonRelationship");
        }
        for body in d["natal"]["placements"].as_array().unwrap() {
            assert!(body["speed"].is_number());
            assert!(body["isRetrograde"].is_boolean());
            assert!(body["distanceAu"].is_number());
        }
        let mut swapped = r.clone();
        swapped["personA"] = r["personB"].clone();
        swapped["personB"] = r["personA"].clone();
        assert_eq!(d["natal"], success(&swapped)["data"]["natal"]);
        if variant == "corrected" {
            let natal = |key: &str| {
                success(
                    &json!({"operation":"natal","utc":r[key]["utc"],"location":r[key]["location"],"houseSystem":r[key]["houseSystem"]}),
                )
            };
            let a = mc(&natal("personA")["data"]);
            let b = mc(&natal("personB")["data"]);
            let target = (a + signed_difference(b, a) / 2.).rem_euclid(360.);
            assert!(signed_difference(mc(&d["natal"]), target).abs() <= 1e-6);
            assert!(p["timeCorrectionSeconds"].as_f64().unwrap().abs() < 43201.);
        } else {
            assert!((p["derivedJulianDayUt1"].as_f64().unwrap() - mean).abs() < 1e-8);
            assert!(p["targetMidheaven"].is_null());
        }
    }
}
#[test]
fn identity_and_arithmetic_dateline_policy() {
    for variant in ["uncorrected", "corrected"] {
        let mut r = request();
        r["personB"] = r["personA"].clone();
        r["variant"] = json!(variant);
        let v = success(&r);
        assert!(
            v["data"]["provenance"]["timeCorrectionSeconds"]
                .as_f64()
                .unwrap()
                .abs()
                < 0.001
        );
        let utc = &v["data"]["natal"]["utc"];
        for key in ["year", "month", "day", "hour", "minute"] {
            assert_eq!(utc[key], r["personA"]["utc"][key]);
        }
    }
    let mut r = request();
    r["personA"]["location"]["longitude"] = json!(179.);
    r["personB"]["location"]["longitude"] = json!(-179.);
    assert_eq!(success(&r)["data"]["natal"]["location"]["longitude"], 0.);
}
#[test]
fn validation_and_selection() {
    for (field, value) in [
        ("variant", json!("spherical")),
        ("variant", Value::Null),
        ("houseSystem", json!("automatic")),
        ("composite", json!({})),
        ("domains", json!([])),
        ("customProfiles", json!([])),
        ("rulership", json!("invalid")),
    ] {
        let mut r = request();
        r[field] = value;
        assert_eq!(run(&r)["errors"][0]["code"], "INVALID_INPUT", "{r}");
    }
    let raw = request().to_string().replacen(
        "\"operation\":\"davison\"",
        "\"operation\":\"davison\",\"variant\":\"corrected\",\"variant\":\"uncorrected\"",
        1,
    );
    let v: Value = serde_json::from_str(&calculate_json(&raw)).unwrap();
    assert_eq!(v["errors"][0]["code"], "INVALID_INPUT");
    let mut r = request();
    r["aspectRules"] = json!([]);
    r["aspectPreset"] = json!("major");
    assert_eq!(run(&r)["errors"][0]["code"], "INVALID_INPUT");
    r.as_object_mut().unwrap().remove("aspectPreset");
    r["houseSystem"] = json!("wholeSign");
    r["domains"] = json!(["love"]);
    let v = success(&r);
    assert_eq!(v["data"]["domains"].as_object().unwrap().len(), 1);
    assert_eq!(v["data"]["natal"]["aspects"], json!([]));
    let mut r = request();
    r["personA"]["utc"]["day"] = json!(32);
    assert_eq!(run(&r)["errors"][0]["code"], "INVALID_INPUT");
}
#[test]
fn antipodal_mc_fails_explicitly() {
    let mut r = request();
    r["personB"] = r["personA"].clone();
    r["personA"]["location"]["longitude"] = json!(0.);
    r["personB"]["location"]["longitude"] = json!(180.);
    r["variant"] = json!("corrected");
    let v = run(&r);
    assert_eq!(v["errors"][0]["code"], "CALCULATION_FAILED");
    assert!(v["errors"][0]["message"]
        .as_str()
        .unwrap()
        .contains("antipodal"));
    assert!(v["data"].is_null());
}

#[test]
fn signed_delta_stays_half_open_at_opposite_angles() {
    for a in [0., 23.52338902475428, 90., 180., 270., 359.99999999999] {
        let b: f64 = (a + 180_f64).rem_euclid(360.);
        for shift in [-1e-13, 0., 1e-13] {
            assert!((-180. ..180.).contains(&signed_difference(a, b + shift)));
            assert!((-180. ..180.).contains(&signed_difference(b + shift, a)));
        }
    }
}
