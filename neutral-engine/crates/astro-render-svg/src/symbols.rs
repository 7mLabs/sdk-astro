use crate::{error, RenderError};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{collections::BTreeMap, sync::OnceLock};

pub const SIGNS: [&str; 12] = [
    "aries",
    "taurus",
    "gemini",
    "cancer",
    "leo",
    "virgo",
    "libra",
    "scorpio",
    "sagittarius",
    "capricorn",
    "aquarius",
    "pisces",
];
pub const BODIES: [&str; 17] = [
    "sun",
    "moon",
    "mercury",
    "venus",
    "mars",
    "jupiter",
    "saturn",
    "uranus",
    "neptune",
    "pluto",
    "chiron",
    "trueNorthNode",
    "trueSouthNode",
    "meanNorthNode",
    "meanSouthNode",
    "meanLilith",
    "osculatingLilith",
];
pub const BODY_PATHS: [&str; 17] = [
    "M16 4A12 12 0 1 0 16 28A12 12 0 1 0 16 4M16 15V17M15 16H17",
    "M21 4A12 12 0 1 0 21 28A13 13 0 0 1 21 4Z",
    "M10 3C10 11 22 11 22 3M16 9A7 7 0 1 0 16 23A7 7 0 1 0 16 9M16 23V30M11 27H21",
    "M16 3A8 8 0 1 0 16 19A8 8 0 1 0 16 3M16 19V30M10 25H22",
    "M12 12A9 9 0 1 0 12 30A9 9 0 1 0 12 12M18 15L29 4M20 4H29V13",
    "M7 7C19 2 15 17 7 18H27M22 3V29",
    "M10 3V28M4 9H19M10 18C22 9 28 17 21 25C18 29 24 30 27 26",
    "M16 4V22M5 5V18M27 5V18M5 11H27M16 21A5 5 0 1 0 16 31A5 5 0 1 0 16 21",
    "M5 5V11C5 24 27 24 27 11V5M16 3V30M10 26H22M2 8L5 4L8 8M24 8L27 4L30 8",
    "M16 3A5 5 0 1 0 16 13A5 5 0 1 0 16 3M6 10C6 24 26 24 26 10M16 22V31M10 27H22",
    "M12 19A5 5 0 1 0 12 29A5 5 0 1 0 12 19M12 19V3M12 12L23 3M12 12L24 18",
    "M6 25C-1 12 7 3 16 3C25 3 33 12 26 25M6 25A4 4 0 1 0 6 17A4 4 0 1 0 6 25M26 25A4 4 0 1 0 26 17A4 4 0 1 0 26 25",
    "M6 7C-1 20 7 29 16 29C25 29 33 20 26 7M6 7A4 4 0 1 0 6 15A4 4 0 1 0 6 7M26 7A4 4 0 1 0 26 15A4 4 0 1 0 26 7",
    "M6 25C-1 12 7 3 16 3C25 3 33 12 26 25M6 25A4 4 0 1 0 6 17A4 4 0 1 0 6 25M26 25A4 4 0 1 0 26 17A4 4 0 1 0 26 25M13 12H19",
    "M6 7C-1 20 7 29 16 29C25 29 33 20 26 7M6 7A4 4 0 1 0 6 15A4 4 0 1 0 6 7M26 7A4 4 0 1 0 26 15A4 4 0 1 0 26 7M13 20H19",
    "M22 3A10 10 0 1 0 22 20A11 11 0 0 1 22 3M16 22V31M10 27H22",
    "M22 3A10 10 0 1 0 22 20A11 11 0 0 1 22 3M16 22V31M10 27H22M26 24H30",

];

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub view_box: [f64; 4],
    pub nodes: Vec<(String, Map<String, Value>)>,
    #[serde(default, flatten)]
    metadata: BTreeMap<String, Value>,
}

pub fn defaults() -> &'static BTreeMap<String, Symbol> {
    static SYMBOLS: OnceLock<BTreeMap<String, Symbol>> = OnceLock::new();
    SYMBOLS.get_or_init(|| {
        let value: Value = serde_json::from_str(include_str!("../assets/zodiac.json")).unwrap();
        value["symbols"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(id, s)| {
                let geometry = serde_json::json!({"viewBox":s["viewBox"],"nodes":s["nodes"]});
                (id.clone(), serde_json::from_value(geometry).unwrap())
            })
            .collect()
    })
}

pub fn validate(overrides: &BTreeMap<String, Symbol>) -> Result<(), RenderError> {
    for (id, s) in overrides {
        for (key, value) in &s.metadata {
            match key.as_str() {
                "label" | "english" | "unicode"
                    if value.as_str().is_some_and(|s| s.chars().count() <= 200) => {}
                "signIndex"
                    if value.as_u64().is_some_and(|n| {
                        n < 12 && SIGNS.get(n as usize).is_some_and(|name| *name == id)
                    }) => {}
                _ => {
                    return Err(error(
                        "INVALID_SYMBOL",
                        "Unknown or invalid symbol metadata",
                    ))
                }
            }
        }
        if !SIGNS.contains(&id.as_str()) && !BODIES.contains(&id.as_str()) {
            return Err(error("INVALID_SYMBOL", format!("Unknown symbol ID: {id}")));
        }
        if s.view_box
            .iter()
            .any(|n| !n.is_finite() || n.abs() > 100_000.)
            || s.view_box[2] <= 0.
            || s.view_box[3] <= 0.
            || s.nodes.is_empty()
            || s.nodes.len() > 64
        {
            return Err(error("INVALID_SYMBOL", "Invalid viewBox or node count"));
        }
        for (tag, attrs) in &s.nodes {
            let allowed: &[&str] = match tag.as_str() {
                "path" => &["d"],
                "circle" => &["cx", "cy", "r"],
                "ellipse" => &["cx", "cy", "rx", "ry"],
                "line" => &["x1", "y1", "x2", "y2"],
                "rect" => &["x", "y", "width", "height", "rx", "ry"],
                "polyline" | "polygon" => &["points"],
                _ => return Err(error("INVALID_SYMBOL", "Unsupported geometry node")),
            };
            if attrs.is_empty() || attrs.keys().any(|key| !allowed.contains(&key.as_str())) {
                return Err(error(
                    "INVALID_SYMBOL",
                    "Only structured geometry attributes are allowed",
                ));
            }
            let required: &[&str] = match tag.as_str() {
                "path" => &["d"],
                "circle" => &["r"],
                "ellipse" => &["rx", "ry"],
                "rect" => &["width", "height"],
                "polygon" | "polyline" => &["points"],
                _ => &[],
            };
            if required.iter().any(|key| !attrs.contains_key(*key)) {
                return Err(error(
                    "INVALID_SYMBOL",
                    "Missing required geometry attribute",
                ));
            }
            for (key, value) in attrs {
                if key == "d" {
                    let path = value
                        .as_str()
                        .ok_or_else(|| error("INVALID_SYMBOL", "Path d must be a string"))?;
                    let segments: Result<Vec<_>, _> = svgtypes::PathParser::from(path).collect();
                    if path.is_empty()
                        || path.len() > 16_384
                        || !path.chars().all(|c| {
                            c.is_ascii()
                                && (c.is_ascii_alphanumeric() || " +-.eE,\t\r\n".contains(c))
                        })
                        || segments
                            .as_ref()
                            .map_or(true, |s| s.is_empty() || s.len() > 2048)
                    {
                        return Err(error("INVALID_SYMBOL", "Invalid or oversized SVG path"));
                    }
                } else if key == "points" {
                    let points = value.as_str().ok_or_else(|| {
                        error("INVALID_SYMBOL", "points must be a numeric string")
                    })?;
                    let nums: Result<Vec<f64>, _> = points
                        .split(|c: char| c == ',' || c.is_ascii_whitespace())
                        .filter(|s| !s.is_empty())
                        .map(str::parse)
                        .collect();
                    if points.len() > 4096
                        || nums.as_ref().map_or(true, |n| {
                            n.len() < 4
                                || n.len() % 2 != 0
                                || n.iter().any(|n| !n.is_finite() || n.abs() > 100_000.)
                        })
                    {
                        return Err(error("INVALID_SYMBOL", "Invalid points"));
                    }
                } else {
                    let n = value.as_f64().ok_or_else(|| {
                        error("INVALID_SYMBOL", "Geometry coordinates must be numbers")
                    })?;
                    if !n.is_finite()
                        || n.abs() > 100_000.
                        || (["r", "rx", "ry", "width", "height"].contains(&key.as_str()) && n < 0.)
                    {
                        return Err(error("INVALID_SYMBOL", "Invalid geometry number"));
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn draw(
    id: &str,
    x: f64,
    y: f64,
    size: f64,
    color: &str,
    overrides: &BTreeMap<String, Symbol>,
) -> String {
    if let Some(symbol) = overrides.get(id).or_else(|| defaults().get(id)) {
        let [vx, vy, w, h] = symbol.view_box;
        let scale = size / w.max(h);
        let mut out=format!("<g transform=\"translate({} {}) scale({}) translate({} {})\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\">",x-w*scale/2.,y-h*scale/2.,scale,-vx,-vy,1.8/32.*w.max(h));
        for (tag, attrs) in &symbol.nodes {
            out.push('<');
            out.push_str(tag);
            for (key, value) in attrs {
                let value = value
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| value.to_string());
                out.push_str(&format!(" {key}=\"{}\"", crate::escape(&value)));
            }
            out.push_str("/>");
        }
        out.push_str("</g>");
        out
    } else if let Some(index) = BODIES.iter().position(|b| *b == id) {
        format!("<path d=\"{}\" transform=\"translate({} {}) scale({})\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",BODY_PATHS[index],x-size/2.,y-size/2.,size/32.)
    } else {
        String::new()
    }
}

pub struct Font {
    units: f64,
    glyphs: Value,
}
impl Font {
    pub fn get() -> &'static Self {
        static FONT: OnceLock<Font> = OnceLock::new();
        FONT.get_or_init(|| {
            let mut v: Value =
                serde_json::from_str(include_str!("../assets/font-outlines.json")).unwrap();
            Font {
                units: v["unitsPerEm"].as_f64().unwrap(),
                glyphs: v["glyphs"].take(),
            }
        })
    }
    pub fn supported(&self, text: &str) -> bool {
        text.chars()
            .all(|c| self.glyphs.get(c.to_string()).is_some())
    }
    pub fn width(&self, text: &str, size: f64) -> f64 {
        text.chars()
            .map(|c| {
                self.glyphs[c.to_string()]["advance"].as_f64().unwrap_or(0.) * size / self.units
            })
            .sum()
    }
    pub fn text(&self, text: &str, x: f64, y: f64, size: f64, color: &str, anchor: &str) -> String {
        let scale = size / self.units;
        let width: f64 = text
            .chars()
            .map(|c| self.glyphs[c.to_string()]["advance"].as_f64().unwrap_or(0.) * scale)
            .sum();
        let mut at = x - match anchor {
            "middle" => width / 2.,
            "end" => width,
            _ => 0.,
        };
        let mut out = format!(
            "<g fill=\"{color}\" aria-label=\"{}\">",
            crate::escape(text)
        );
        for c in text.chars() {
            let glyph = &self.glyphs[c.to_string()];
            if let Some(path) = glyph["path"].as_str() {
                if !path.is_empty() {
                    out.push_str(&format!(
                        "<path d=\"{path}\" transform=\"translate({at} {y}) scale({scale} {})\"/>",
                        -scale
                    ));
                }
            }
            at += glyph["advance"].as_f64().unwrap_or(0.) * scale;
        }
        out.push_str("</g>");
        out
    }
}
