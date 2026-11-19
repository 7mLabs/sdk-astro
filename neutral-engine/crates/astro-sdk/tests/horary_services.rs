use serde_json::{json, Value};

#[test]
fn horary_local_chart_renders_its_selected_house_and_roundtrips_losslessly() {
    let request = json!({
        "operation":"horary","quesitedHouse":7,
        "moment":{
            "local":{"year":2026,"month":11,"day":19,"hour":9,"minute":30},
            "timeZone":"Asia/Ho_Chi_Minh","location":{"latitude":10.82,"longitude":106.63}
        }
    });
    let payload: Value =
        serde_json::from_str(&astro_sdk::calculate_from_local_json(&request.to_string())).unwrap();
    assert_eq!(payload["errors"], json!([]));
    assert_eq!(payload["data"]["chart"]["utc"]["hour"], 2);
    let render = |options| -> Value {
        let request = json!({"payload":payload,"options":options});
        serde_json::from_str(&astro_sdk::render_chart_svg_json(&request.to_string())).unwrap()
    };
    let rendered =
        render(json!({"sections":["wheel","houseFocus"],"focus":{"house":7,"layer":"Q"}}));
    assert_eq!(rendered["errors"], json!([]));
    let view = &rendered["data"];
    assert!(view["svg"].as_str().unwrap().starts_with("<svg"));
    assert_eq!(view["metadata"]["chart"], "horary");
    assert_eq!(
        view["metadata"]["sourceCalculation"]["ruleset"],
        "western-traditional-v1"
    );
    assert_eq!(view["coverage"]["renderedBodies"], 7);
    assert_eq!(view["model"]["houseContextAvailable"], true);
    assert_eq!(
        view["model"]["focus"]["house"]["rulerId"],
        payload["data"]["significators"]["quesited"]["rulerBodyId"]
    );
    let explicit = render(json!({"chart":"horary","output":"model"}));
    assert_eq!(explicit["errors"], json!([]));
    assert_eq!(explicit["data"]["model"]["chart"], "horary");
    let compressed = astro_core::compress_json(&json!({"payload":payload}).to_string());
    let restored: Value =
        serde_json::from_str(&astro_core::expand_context_json(&compressed)).unwrap();
    assert_eq!(restored, payload);
}

#[test]
fn polar_horary_preserves_clockwise_houses_in_svg_and_hit_targets() {
    let input = json!({"operation":"horary","quesitedHouse":7,"moment":{
        "utc":{"year":2026,"month":11,"day":19,"hour":16,"minute":59,"second":59},
        "location":{"latitude":78.2232,"longitude":15.6469},"houseSystem":"regiomontanus"}});
    let payload: Value =
        serde_json::from_str(&astro_core::calculate_json(&input.to_string())).unwrap();
    assert_eq!(payload["errors"], json!([]));
    assert_eq!(payload["calculation"]["houseOrder"], "decreasingLongitude");
    let chart = &payload["data"]["chart"];
    let cusps: Vec<_> = chart["houseCusps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c.as_f64().unwrap())
        .collect();
    let wraps_zero = (0..12).find(|&i| cusps[(i + 1) % 12] > cusps[i]).unwrap();
    let focus_house = wraps_zero + 1;
    let render = |p: &Value| -> Value {
        let req = json!({"payload":p, "options":{"orientation":"aries",
            "sections":["wheel","houseFocus"],"focus":{"house":focus_house,"layer":"Q"}}});
        serde_json::from_str(&astro_sdk::render_chart_svg_json(&req.to_string())).unwrap()
    };
    let result = render(&payload);
    assert_eq!(result["errors"], json!([]));
    let view = &result["data"];
    let model = &view["model"];
    let svg = view["svg"].as_str().unwrap();
    assert_eq!(
        view["metadata"]["geometry"][0]["houseCusps"],
        chart["houseCusps"]
    );
    assert_eq!(model["houses"].as_array().unwrap().len(), 12);
    let mut total = 0.;
    for (i, house) in model["houses"].as_array().unwrap().iter().enumerate() {
        let start = cusps[i];
        let width = (start - cusps[(i + 1) % 12]).rem_euclid(360.);
        total += width;
        assert_eq!(house["longitude"], start);
        assert_eq!(house["number"], i + 1);
        assert_eq!(house["width"], width);
        assert_eq!(house["signedWidth"], -width);
        assert_eq!(house["rulerId"], chart["houses"][i]["rulerBodyId"]);
        let target = model["hitTargets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|target| target["kind"] == "house" && target["house"] == i + 1)
            .unwrap();
        let mid = (180. - (start - width / 2.)).to_radians();
        let local_y = 520. + 195. * mid.sin();
        assert!(
            (target["center"]["x"].as_f64().unwrap() - (500. + 1.08 * 195. * mid.cos())).abs()
                < 1e-8
        );
        assert!(
            (target["center"]["y"].as_f64().unwrap() - (565. + 1.08 * 195. * mid.sin())).abs()
                < 1e-8
        );
        // Labels are outlined glyphs. Compare the SVG glyph baseline to
        // the very same clockwise sector represented by the model hit target.
        let label = format!("aria-label=\"{}\">", i + 1);
        let group = svg
            .split_once(&label)
            .unwrap()
            .1
            .split_once("</g>")
            .unwrap()
            .0;
        let translate = group
            .split_once("translate(")
            .unwrap()
            .1
            .split_once(')')
            .unwrap()
            .0;
        let label_y: f64 = translate
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        assert!((label_y - (local_y + 5.)).abs() < 1e-8);
    }
    assert!((total - 360.).abs() < 1e-7);
    assert_eq!(model["focus"]["house"], model["houses"][wraps_zero]);
    let path = svg
        .split_once(&format!("data-focus-house=\"{focus_house}\" d=\""))
        .unwrap()
        .1
        .split_once('"')
        .unwrap()
        .0;
    let tokens: Vec<_> = path.split_whitespace().collect();
    // Clockwise outer sweep to the numbered next cusp; return on the inner arc.
    assert_eq!(&tokens[2..7], &["A222", "222", "0", "0", "1"]);
    assert_eq!(&tokens[11..16], &["A178", "178", "0", "0", "0"]);
    let end = (180. - cusps[(wraps_zero + 1) % 12]).to_radians();
    assert!((tokens[7].parse::<f64>().unwrap() - (500. + 222. * end.cos())).abs() < 1e-8);
    assert!((tokens[8].parse::<f64>().unwrap() - (520. + 222. * end.sin())).abs() < 1e-8);

    let mut mismatched = payload.clone();
    mismatched["calculation"]["houseOrder"] = json!("increasingLongitude");
    assert_eq!(
        render(&mismatched)["errors"][0]["code"],
        "INVALID_CHART_DATA"
    );
    mismatched["calculation"]["houseOrder"] = json!("decreasingLongitude");
    mismatched["calculation"]["houseSystem"] = json!("wholeSign");
    assert_eq!(
        render(&mismatched)["errors"][0]["code"],
        "INVALID_CHART_DATA"
    );
}
