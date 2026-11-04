use astro_core::{calculate, calculate_json, Request};
use serde_json::{json, Value};

fn run(v: Value) -> Value {
    serde_json::from_str(&calculate_json(&v.to_string())).unwrap()
}
fn birth() -> Value {
    json!({"utc":{"year":2000,"month":1,"day":1,"hour":12,"minute":0},
        "location":{"latitude":10.8231,"longitude":106.6297},"houseSystem":"wholeSign"})
}

#[test]
fn legacy_rust_request_literal_and_unknown_geometry_provenance() {
    let request = Request {
        operation: "chart".into(),
        positions: vec![],
        other_positions: None,
        houses: None,
        other_houses: None,
        harmonic: None,
        aspect_rules: vec![],
    };
    assert!(calculate(request).is_err());
    let v = run(json!({"operation":"chart","positions":[{"id":"sun","longitude":10}]}));
    assert_eq!(v["calculation"]["operation"], "chart");
    assert_eq!(v["calculation"]["chartKind"], "suppliedChartGeometry");
    assert_eq!(
        v["calculation"]["inputProvenance"]["positions"],
        json!({"status":"unspecified","verified":false,"context":null})
    );
    assert!(v["calculation"].get("zodiac").is_none());
}

#[test]
fn declarations_remain_unverified_and_do_not_change_geometry() {
    let base = json!({"operation":"synastry","positions":[{"id":"sun","longitude":10}],
        "otherPositions":[{"id":"moon","longitude":130}]});
    let mut declared = base.clone();
    declared["sourceContext"] = json!({"zodiac":"tropical","sourceId":"test","sourceVersion":"1"});
    declared["otherSourceContext"] = json!({"zodiac":"sidereal","ayanamsa":"caller-method"});
    let a = run(base);
    let b = run(declared);
    assert_eq!(a["data"], b["data"]);
    let p = &b["calculation"]["inputProvenance"];
    assert_eq!(p["positions"]["context"]["zodiac"], "tropical");
    assert_eq!(p["otherPositions"]["context"]["ayanamsa"], "caller-method");
    assert_eq!(p["positions"]["status"], "callerDeclared");
    assert_eq!(p["otherPositions"]["verified"], false);
}

#[test]
fn invalid_declarations_are_rejected_without_silent_defaults() {
    for context in [
        Value::Null,
        json!({}),
        json!({"zodiac":"unknown"}),
        json!({"sourceId":" "}),
        json!({"sourceId":"x".repeat(129)}),
        json!({"zodiac":"tropical","ayanamsa":"x"}),
        json!({"unknown":true}),
        json!({"coordinates":"barycentric"}),
    ] {
        let v = run(
            json!({"operation":"chart","positions":[{"id":"sun","longitude":10}],"sourceContext":context}),
        );
        assert_eq!(v["data"], Value::Null, "{context}");
        assert_eq!(v["errors"][0]["code"], "INVALID_INPUT");
    }
    let v = run(
        json!({"operation":"chart","positions":[{"id":"sun","longitude":10}],"otherSourceContext":{"zodiac":"tropical"}}),
    );
    assert_eq!(v["errors"][0]["code"], "INVALID_INPUT");
}

#[test]
fn query_retains_source_and_rules_without_inventing_a_school() {
    let b = birth();
    let v = run(
        json!({"operation":"query","group":"houses","action":"inspect",
        "birth":b,"houseNumbers":[7],"rulership":"modern","includeMetadata":true}),
    );
    assert_eq!(v["errors"], json!([]));
    assert_eq!(
        v["calculation"]["sourceBirth"],
        json!({"utc":{"year":2000,"month":1,"day":1,"hour":12,"minute":0,"second":0.0},
        "location":{"latitude":10.8231,"longitude":106.6297},"houseSystem":"wholeSign"})
    );
    assert_eq!(v["calculation"]["rulershipRulesVersion"], "1.0");
    assert_eq!(v["calculation"]["houseAssignment"], "eclipticLongitude");
    assert_eq!(v["calculation"]["operation"], "query");
    let n = run(
        json!({"operation":"natalDomains","utc":b["utc"],"location":b["location"],"rulership":"modern","domains":["career"]}),
    );
    assert_eq!(
        n["data"]["context"]["advanced"]["rules"]["dignity"]["system"],
        "traditional"
    );
    assert_eq!(n["calculation"]["rulership"], "modern");
}

#[test]
fn default_queries_keep_closed_legacy_metadata_contract() {
    for request in [
        json!({"operation":"query","group":"geometry","action":"normalize","longitude":10}),
        json!({"operation":"query","group":"houses","action":"inspect","birth":birth(),"houseNumbers":[7]}),
    ] {
        let legacy = run(request.clone());
        let mut enriched = request.clone();
        enriched["includeMetadata"] = json!(true);
        let enriched = run(enriched);
        assert_eq!(legacy["data"], enriched["data"]);
        assert!(legacy["calculation"].get("operation").is_none());
        assert!(legacy["calculation"].get("sourceBirth").is_none());
        assert_eq!(enriched["calculation"]["operation"], "query");
        let mut invalid = request;
        invalid["includeMetadata"] = Value::Null;
        assert_eq!(run(invalid)["errors"][0]["code"], "INVALID_INPUT");
    }
}
