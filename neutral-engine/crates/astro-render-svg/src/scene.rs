use crate::{error, RenderError};
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Point {
    pub id: String,
    pub body: String,
    pub longitude: f64,
    pub retrograde: Option<bool>,
    pub house: Option<u64>,
}
pub struct Layer {
    pub key: String,
    pub points: Vec<Point>,
    pub houses: Vec<f64>,
    pub angles: Vec<(String, f64)>,
}
pub struct Scene {
    pub key: String,
    pub layers: Vec<Layer>,
    pub aspects: Vec<Value>,
    pub house_system: String,
}

fn longitude(v: &Value) -> Result<f64, RenderError> {
    let n = v
        .as_f64()
        .ok_or_else(|| error("INVALID_CHART_DATA", "Missing numeric longitude"))?;
    if !n.is_finite() || !(0.0..360.0).contains(&n) {
        return Err(error("INVALID_CHART_DATA", "longitude must be in [0,360)"));
    }
    Ok(n)
}
fn layer(
    chart: &Value,
    key: &str,
    qualified: bool,
    allow_no_houses: bool,
) -> Result<Layer, RenderError> {
    let values = chart["placements"]
        .as_array()
        .or_else(|| chart["positions"].as_array())
        .ok_or_else(|| error("MISSING_CHART_DATA", "Chart requires placements/positions"))?;
    let mut values: Vec<_> = values.iter().collect();
    if let Some(extra) = chart.get("additionalPoints") {
        let extra = extra
            .as_array()
            .ok_or_else(|| error("INVALID_CHART_DATA", "additionalPoints must be an array"))?;
        values.extend(extra);
    }
    if values.is_empty() || values.len() > 32 {
        return Err(error(
            "INVALID_CHART_DATA",
            "Chart requires 1..32 placements",
        ));
    }
    let mut seen = BTreeSet::new();
    let points = values
        .iter()
        .map(|v| {
            let id = v["id"]
                .as_str()
                .ok_or_else(|| error("INVALID_CHART_DATA", "Placement requires id"))?;
            if !super::symbols::BODIES.contains(&id) || !seen.insert(id) {
                return Err(error(
                    "INVALID_CHART_DATA",
                    "Unsupported or duplicate body ID",
                ));
            }
            let house = v["house"].as_u64();
            if house.is_some_and(|h| !(1..=12).contains(&h)) {
                return Err(error("INVALID_CHART_DATA", "Invalid house number"));
            }
            if !v["isRetrograde"].is_null() && !v["isRetrograde"].is_boolean() {
                return Err(error("INVALID_CHART_DATA", "Invalid retrograde value"));
            }
            Ok(Point {
                id: if qualified {
                    format!("{key}:{id}")
                } else {
                    id.to_owned()
                },
                body: id.into(),
                longitude: longitude(&v["longitude"])?,
                retrograde: v["isRetrograde"].as_bool(),
                house,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let houses = if let Some(cusps) = chart["houseCusps"].as_array() {
        cusps.iter().map(longitude).collect::<Result<Vec<_>, _>>()?
    } else if let Some(houses) = chart["houses"].as_array() {
        houses
            .iter()
            .map(|h| longitude(&h["longitude"]))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![]
    };
    if houses.len() != 12 && !(allow_no_houses && houses.is_empty()) {
        return Err(error(
            "MISSING_CHART_DATA",
            "Chart requires exactly 12 house cusps",
        ));
    }
    if !houses.is_empty() {
        let widths: Vec<f64> = (0..12)
            .map(|i| (houses[(i + 1) % 12] - houses[i]).rem_euclid(360.))
            .collect();
        if widths.iter().any(|w| *w <= 0.) || (widths.iter().sum::<f64>() - 360.).abs() > 1e-7 {
            return Err(error(
                "INVALID_CHART_DATA",
                "House cusps must form one ordered nondegenerate zodiac cycle",
            ));
        }
    }
    let mut angles = vec![];
    let mut seen = BTreeSet::new();
    if let Some(values) = chart["angles"].as_array() {
        for v in values {
            let id = v["id"].as_str().unwrap_or("");
            if !["ascendant", "midheaven", "descendant", "imumCoeli"].contains(&id)
                || !seen.insert(id)
            {
                return Err(error("INVALID_CHART_DATA", "Invalid or duplicate angle ID"));
            }
            angles.push((id.to_owned(), longitude(&v["longitude"])?));
        }
    }
    Ok(Layer {
        key: key.into(),
        points,
        houses,
        angles,
    })
}
fn aspects(chart: &Value, context: &Value) -> Vec<Value> {
    context["aspects"]
        .as_array()
        .or_else(|| chart["aspects"].as_array())
        .cloned()
        .unwrap_or_default()
}

pub fn parse(payload: &Value, requested: Option<&str>) -> Result<Scene, RenderError> {
    if payload["schemaVersion"] != "1.0"
        || !payload["errors"].as_array().is_some_and(|a| a.is_empty())
        || !payload["data"].is_object()
    {
        return Err(error(
            "INVALID_PAYLOAD",
            "Expected a successful SDK schemaVersion 1.0 result envelope",
        ));
    }
    let d = &payload["data"];
    let key = requested.unwrap_or(if d["composite"]["chart"].is_object() {
        "C"
    } else if d["subjects"]["A"].is_object() {
        "synastry"
    } else if d["subject"]["natal"].is_object() {
        "transits"
    } else {
        "natal"
    });
    let mut layers = vec![];
    let contacts = match key {
        "natal" => {
            let chart = if d["natal"].is_object() {
                &d["natal"]
            } else {
                d
            };
            layers.push(layer(chart, "N", false, false)?);
            aspects(chart, &d["context"])
        }
        "A" | "B" => {
            let subject = &d["subjects"][key];
            layers.push(layer(&subject["natal"], key, false, false)?);
            aspects(&subject["natal"], &subject["context"])
        }
        "C" => {
            let c = &d["composite"];
            layers.push(layer(&c["chart"], "C", false, false)?);
            aspects(&c["chart"], &c["context"])
        }
        "synastry" => {
            for key in ["A", "B"] {
                layers.push(layer(&d["subjects"][key]["natal"], key, true, false)?);
            }
            d["context"]["aspects"]
                .as_array()
                .cloned()
                .ok_or_else(|| error("MISSING_CHART_DATA", "No cross-chart aspects"))?
        }
        "transits" => {
            layers.push(layer(&d["subject"]["natal"], "N", true, false)?);
            layers.push(layer(&d["snapshot"], "T", true, true)?);
            d["snapshot"]["aspects"]
                .as_array()
                .cloned()
                .ok_or_else(|| error("MISSING_CHART_DATA", "No transit snapshot aspects"))?
        }
        _ => {
            return Err(error(
                "UNSUPPORTED_CHART",
                "chart must be natal, A, B, C, synastry or transits",
            ))
        }
    };
    if contacts.len() > 4096 {
        return Err(error("INVALID_CHART_DATA", "Aspect budget exceeded"));
    }
    let mut seen = BTreeSet::new();
    for aspect in &contacts {
        let angle = aspect["angle"]
            .as_f64()
            .ok_or_else(|| error("INVALID_CHART_DATA", "Aspect requires numeric angle"))?;
        if !angle.is_finite() || !(0.0..=180.).contains(&angle) {
            return Err(error("INVALID_CHART_DATA", "Invalid aspect angle"));
        }
        if let Some(index) = aspect["index"].as_u64() {
            if !seen.insert(index) {
                return Err(error("INVALID_CHART_DATA", "Duplicate aspect index"));
            }
        }
    }
    let house_system = payload["calculation"]["houseSystem"]
        .as_str()
        .or_else(|| d["subjects"]["A"]["calculation"]["houseSystem"].as_str())
        .or_else(|| d["subject"]["calculation"]["houseSystem"].as_str())
        .unwrap_or("source-defined")
        .to_owned();
    Ok(Scene {
        key: key.into(),
        layers,
        aspects: contacts,
        house_system,
    })
}

pub fn endpoint(v: &Value, first: bool, chart: &str) -> Option<String> {
    let keys = if first {
        ["point1", "body1", "transitPointId"]
    } else {
        ["point2", "body2", "natalPointId"]
    };
    keys.iter().find_map(|key| {
        v[*key].as_str().map(|id| {
            if chart == "transits" && !id.contains(':') {
                format!("{}:{id}", if first { "T" } else { "N" })
            } else {
                id.into()
            }
        })
    })
}

pub fn source_geometry(scene: &Scene) -> Value {
    json!(scene.layers.iter().map(|l|json!({"layer":l.key,"houseCusps":l.houses,"angles":l.angles.iter().map(|(id,n)|json!({"id":id,"longitude":n})).collect::<Vec<_>>(),"placements":l.points.iter().map(|p|json!({"id":p.id,"longitude":p.longitude,"isRetrograde":p.retrograde,"house":p.house})).collect::<Vec<_>>()})).collect::<Vec<_>>())
}
