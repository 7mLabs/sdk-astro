//! Optional SDK services composed around the UTC-only calculation core.
use serde::Deserialize;
use serde::{
    de::{Error as _, MapAccess, Visitor},
    Deserializer,
};
use serde_json::value::RawValue;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fmt};

// Preserve raw literals in fields passed through to the existing core, including
// the other subject's UTC. Never round a tiny invalid integer fraction to a year.
struct RawObject(BTreeMap<String, Box<RawValue>>);
impl<'de> Deserialize<'de> for RawObject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ObjectVisitor;
        impl<'de> Visitor<'de> for ObjectVisitor {
            type Value = RawObject;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a calculation object with unique fields")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut map = BTreeMap::new();
                while let Some((key, value)) = access.next_entry::<String, Box<RawValue>>()? {
                    if map.insert(key, value).is_some() {
                        return Err(A::Error::custom("Duplicate calculation field"));
                    }
                }
                Ok(RawObject(map))
            }
        }
        deserializer.deserialize_map(ObjectVisitor)
    }
}

pub const MAX_RENDER_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_TIME_BYTES: usize = 16 * 1024;
fn result(scope: &str, data: Value, warnings: Value) -> String {
    json!({"schemaVersion":"1.0","engineVersion":astro_core::VERSION,"calculation":{"scope":scope},"data":data,"warnings":warnings,"errors":[]}).to_string()
}
fn input(text: &str, max: usize) -> Result<Value, String> {
    if text.len() > max {
        return Err(astro_core::error_json(
            "INPUT_TOO_LARGE",
            "Input exceeds this service's byte limit",
        ));
    }
    serde_json::from_str(text).map_err(|e| astro_core::error_json("INVALID_JSON", &e.to_string()))
}

pub fn resolve_birth_time_json(text: &str) -> String {
    match input(text, MAX_TIME_BYTES) {
        Ok(_) => {}
        Err(e) => return e,
    };
    match astro_time::resolve_json(text) {
        Ok(data) => {
            let warnings = data["warnings"].clone();
            result("civil-time-resolution", data, warnings)
        }
        Err(e) => astro_core::error_json(e.code, &e.message),
    }
}

/// Accept a local birth at the root, personA/personB, or birth (forecast/query).
/// UTC-only callers keep using astro_core::calculate_json without this helper.
pub fn calculate_from_local_json(text: &str) -> String {
    match input(text, astro_core::MAX_REQUEST_BYTES) {
        Ok(_) => {}
        Err(e) => return e,
    };
    let mut root = match serde_json::from_str::<RawObject>(text) {
        Ok(v) => v.0,
        Err(e) => return astro_core::error_json("INVALID_TIME_INPUT", &e.to_string()),
    };
    root.entry("operation".into())
        .or_insert_with(|| RawValue::from_string("\"natal\"".into()).unwrap());
    let operation = serde_json::from_str::<String>(root["operation"].get()).unwrap_or_default();
    let paths:Vec<&str>=match operation.as_str() {
        "natal"|"natalDomains"=>vec![""],"couple"|"composite"=>vec!["personA","personB"],
        "forecast"|"query"=>vec!["birth"],_=>return astro_core::error_json("UNSUPPORTED_LOCAL_OPERATION","Local-time calculation supports natal, natalDomains, couple, composite, forecast and birth-based query"),
    };
    let mut resolutions = serde_json::Map::new();
    let mut warnings = vec![];
    for path in paths {
        let mut object = if path.is_empty() {
            std::mem::take(&mut root)
        } else {
            match root.get(path) {
                Some(v) => match serde_json::from_str::<RawObject>(v.get()) {
                    Ok(o) => o.0,
                    Err(e) => return astro_core::error_json("INVALID_TIME_INPUT", &e.to_string()),
                },
                None => continue,
            }
        };
        if !object.contains_key("local") {
            if path.is_empty() {
                root = object;
            }
            continue;
        }
        if object.contains_key("utc") {
            return astro_core::error_json(
                "INVALID_TIME_INPUT",
                "Do not provide both local and utc for the same birth",
            );
        }
        let mut request: BTreeMap<String, Box<RawValue>> = BTreeMap::new();
        for key in ["local", "timeZone", "utcOffsetSeconds", "disambiguation"] {
            if let Some(v) = object.remove(key) {
                request.insert(key.into(), v);
            }
        }
        let resolved = match astro_time::resolve_json(&serde_json::to_string(&request).unwrap()) {
            Ok(v) => v,
            Err(e) => return astro_core::error_json(e.code, &e.message),
        };
        object.insert(
            "utc".into(),
            RawValue::from_string(resolved["utc"].to_string()).unwrap(),
        );
        if path.is_empty() {
            root = object;
        } else {
            root.insert(
                path.into(),
                RawValue::from_string(serde_json::to_string(&object).unwrap()).unwrap(),
            );
        }
        if let Some(w) = resolved["warnings"].as_array() {
            warnings.extend(w.iter().cloned());
        }
        resolutions.insert(
            if path.is_empty() { "birth" } else { path }.into(),
            resolved,
        );
    }
    if resolutions.is_empty() {
        return astro_core::error_json(
            "INVALID_TIME_INPUT",
            "No local birth was supplied. Use calculate for an already-UTC request.",
        );
    }
    let mut calculated: Value = serde_json::from_str(&astro_core::calculate_json(
        &serde_json::to_string(&root).unwrap(),
    ))
    .unwrap();
    // Only attach resolution to a successful calculation. Original errors retain their contract.
    if calculated["errors"]
        .as_array()
        .is_some_and(|e| e.is_empty())
    {
        calculated["calculation"]["timeResolution"] = Value::Object(resolutions);
        if let Some(w) = calculated["warnings"].as_array_mut() {
            for warning in warnings {
                if !w.contains(&warning) {
                    w.push(warning);
                }
            }
        }
    }
    calculated.to_string()
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RenderRequest {
    payload: Value,
    #[serde(default)]
    options: astro_render_svg::Options,
}
pub fn render_chart_svg_json(text: &str) -> String {
    let value = match input(text, MAX_RENDER_BYTES) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let request: RenderRequest = match serde_json::from_value(value) {
        Ok(v) => v,
        Err(e) => return astro_core::error_json("INVALID_RENDER_OPTIONS", &e.to_string()),
    };
    match astro_render_svg::render(&request.payload, request.options) {
        Ok(data) => {
            let warnings = data["warnings"].clone();
            result("svg-chart-rendering", data, warnings)
        }
        Err(e) => astro_core::error_json(e.code, &e.message),
    }
}
pub fn capabilities_json(_: &str) -> String {
    result(
        "sdk-capabilities",
        json!({"abiVersion":1,"civilTime":{"iana":true,"tzdbVersion":bundled_tzdb_version(),"explicitOffsetSeconds":true,"ambiguousTime":"reject by default; earlier/later opt-in","nonexistentTime":"reject"},"svg":{"rendererVersion":astro_render_svg::RENDERER_VERSION,"details":["basic","advanced","professional"],"charts":["natal","A","B","C","synastry","transits"],"selfContained":true,"customZodiacSymbols":true,"customPlanetSymbols":true,"aspectColorGroups":["harmonious","challenging","neutral","minor","other"],"displayOverrides":true,"parts":["wheel","positions","houses","aspectMatrix","aspects","houseFocus"],"modelVersion":"1.0","houseFocus":true},"additionalPoints":{"operations":["natal","natalDomains"],"ids":["chiron","trueNorthNode","trueSouthNode","meanNorthNode","meanSouthNode","meanLilith","osculatingLilith"],"presets":["standard","all"],"embeddedChiron":true},"browserWasm":false}),
        json!([]),
    )
}
fn bundled_tzdb_version() -> Value {
    let probe = astro_time::resolve(
        &json!({"local":{"year":2000,"month":1,"day":1,"hour":0,"minute":0},"timeZone":"Etc/UTC"}),
    )
    .unwrap();
    probe["tzdbVersion"].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_and_manual_utc_produce_same_calculation() {
        let req = json!({"operation":"natal","local":{"year":1995,"month":6,"day":15,"hour":14,"minute":30},"timeZone":"Asia/Ho_Chi_Minh","location":{"latitude":10.8,"longitude":106.6}});
        let out: Value =
            serde_json::from_str(&calculate_from_local_json(&req.to_string())).unwrap();
        assert_eq!(out["errors"], json!([]));
        let utc = out["calculation"]["timeResolution"]["birth"]["utc"].clone();
        let expected: Value = serde_json::from_str(&astro_core::calculate_json(
            &json!({"operation":"natal","utc":utc,"location":req["location"]}).to_string(),
        ))
        .unwrap();
        assert_eq!(out["data"], expected["data"]);
        let rendered: Value = serde_json::from_str(&render_chart_svg_json(
            &json!({"payload":out,"options":{"detail":"advanced"}}).to_string(),
        ))
        .unwrap();
        assert_eq!(rendered["errors"], json!([]));
        assert!(rendered["data"]["svg"]
            .as_str()
            .unwrap()
            .starts_with("<svg"));
    }
    #[test]
    fn invalid_times_and_missing_helpers_do_not_calculate() {
        let req = json!({"local":{"year":2024,"month":3,"day":10,"hour":2,"minute":30},"timeZone":"America/New_York","location":{"latitude":40.,"longitude":-74.}});
        let out: Value =
            serde_json::from_str(&calculate_from_local_json(&req.to_string())).unwrap();
        assert_eq!(out["errors"][0]["code"], "NONEXISTENT_LOCAL_TIME");
        let both = json!({"operation":"natal","local":{},"utc":{}});
        assert!(calculate_from_local_json(&both.to_string()).contains("INVALID_TIME_INPUT"));
        assert!(calculate_from_local_json("{}").contains("INVALID_TIME_INPUT"));
    }
    #[test]
    fn services_reject_bad_json_and_oversized_input() {
        assert!(resolve_birth_time_json("{").contains("INVALID_JSON"));
        assert!(
            resolve_birth_time_json(&" ".repeat(MAX_TIME_BYTES + 1)).contains("INPUT_TOO_LARGE")
        );
        assert!(render_chart_svg_json("{\"payload\":null}").contains("INVALID_PAYLOAD"));
        assert!(capabilities_json("{}").contains("tzdbVersion"));
    }
    #[test]
    fn integer_literals_are_not_rounded_before_validation() {
        let raw = r#"{"local":{"year":1995.0000000000000001,"month":6,"day":15,"hour":14,"minute":30},"timeZone":"Asia/Ho_Chi_Minh"}"#;
        let response: Value = serde_json::from_str(&resolve_birth_time_json(raw)).unwrap();
        assert!(!response["errors"].as_array().unwrap().is_empty());
        let request = r#"{"operation":"couple","personA":{"local":{"year":1995,"month":6,"day":15,"hour":14,"minute":30},"timeZone":"Asia/Ho_Chi_Minh","location":{"latitude":0,"longitude":0}},"personB":{"utc":{"year":2000.0000000000000001,"month":1,"day":1,"hour":12,"minute":0},"location":{"latitude":0,"longitude":0}}}"#;
        let response: Value = serde_json::from_str(&calculate_from_local_json(request)).unwrap();
        assert!(!response["errors"].as_array().unwrap().is_empty());
        assert!(
            calculate_from_local_json(r#"{"operation":"natal","operation":"couple"}"#)
                .contains("Duplicate calculation field")
        );
    }
}
