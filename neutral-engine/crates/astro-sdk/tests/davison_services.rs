use serde_json::{json, Value};
#[test]
fn both_variants_support_local_time_svg_model_and_lossless_context() {
    for variant in ["uncorrected", "corrected"] {
        let r = json!({"operation":"davison","variant":variant,"personA":{"local":{"year":1985,"month":5,"day":8,"hour":14,"minute":30},"timeZone":"Asia/Ho_Chi_Minh","location":{"latitude":10.8,"longitude":106.6}},"personB":{"utc":{"year":1990,"month":8,"day":12,"hour":5,"minute":10},"location":{"latitude":21.,"longitude":105.8}},"domains":["love"]});
        let v: Value =
            serde_json::from_str(&astro_sdk::calculate_from_local_json(&r.to_string())).unwrap();
        assert_eq!(v["errors"], json!([]));
        assert!(v["calculation"]["timeResolution"]["personA"].is_object());
        let render = json!({"payload":v,"options":{"sections":["wheel","houseFocus"],"focus":{"house":7,"mode":"highlight"}}});
        let svg: Value =
            serde_json::from_str(&astro_sdk::render_chart_svg_json(&render.to_string())).unwrap();
        assert_eq!(svg["errors"], json!([]));
        assert!(svg["data"]["svg"].as_str().unwrap().starts_with("<svg"));
        let encoded: Value = serde_json::from_str(&astro_core::compress_json(
            &json!({"payload":v,"options":{}}).to_string(),
        ))
        .unwrap();
        assert_eq!(encoded["errors"], json!([]));
        let expanded: Value =
            serde_json::from_str(&astro_core::expand_context_json(&encoded.to_string())).unwrap();
        assert_eq!(expanded["errors"], json!([]));
        assert_eq!(expanded, v);
    }
}
