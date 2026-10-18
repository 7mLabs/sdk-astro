use crate::{error, RenderError};
use std::collections::BTreeMap;

pub const COLOR_KEYS: [&str; 16] = [
    "harmonious",
    "challenging",
    "neutral",
    "minor",
    "other",
    "conjunction",
    "sextile",
    "square",
    "trine",
    "opposition",
    "semisextile",
    "semisquare",
    "quintile",
    "sesquiquadrate",
    "biquintile",
    "quincunx",
];
pub fn validate(colors: &BTreeMap<String, String>) -> Result<(), RenderError> {
    for (key, value) in colors {
        if !COLOR_KEYS.contains(&key.as_str())
            || ![4, 5, 7, 9].contains(&value.len())
            || !value.starts_with('#')
            || !value[1..].bytes().all(|c| c.is_ascii_hexdigit())
        {
            return Err(error("INVALID_RENDER_OPTIONS", "aspectColors requires known group/type keys and #RGB/#RGBA/#RRGGBB/#RRGGBBAA values"));
        }
    }
    Ok(())
}
pub fn kind(angle: f64) -> (&'static str, &'static str) {
    match angle {
        0. => ("conjunction", "neutral"),
        60. => ("sextile", "harmonious"),
        120. => ("trine", "harmonious"),
        90. => ("square", "challenging"),
        180. => ("opposition", "challenging"),
        30. => ("semisextile", "minor"),
        45. => ("semisquare", "minor"),
        72. => ("quintile", "minor"),
        135. => ("sesquiquadrate", "minor"),
        144. => ("biquintile", "minor"),
        150. => ("quincunx", "minor"),
        _ => ("other", "other"),
    }
}
pub fn color(angle: f64, light: bool, colors: &BTreeMap<String, String>) -> &str {
    let (name, group) = kind(angle);
    if let Some(c) = colors.get(name).or_else(|| colors.get(group)) {
        return c;
    }
    match angle {
        60. => {
            if light {
                "#145BFF"
            } else {
                "#74ADFF"
            }
        }
        120. => {
            if light {
                "#15754B"
            } else {
                "#5DD6A0"
            }
        }
        90. => {
            if light {
                "#C52E48"
            } else {
                "#FF7189"
            }
        }
        180. => {
            if light {
                "#AF4C18"
            } else {
                "#FFB36B"
            }
        }
        0. => {
            if light {
                "#7745D9"
            } else {
                "#B09AFF"
            }
        }
        _ => {
            if light {
                "#667084"
            } else {
                "#A2AEC8"
            }
        }
    }
}
pub fn line_attributes(angle: f64) -> &'static str {
    match angle {
        0. => "stroke-dasharray=\"2 3\"",
        60. => "stroke-dasharray=\"6 3\"",
        90. => "stroke-dasharray=\"10 3\"",
        120. | 180. => "",
        _ => "stroke-dasharray=\"2 4\"",
    }
}
pub fn name(angle: f64, vi: bool) -> &'static str {
    match (angle, vi) {
        (0., true) => "Trùng tụ",
        (60., true) => "Lục hợp",
        (90., true) => "Vuông",
        (120., true) => "Tam hợp",
        (180., true) => "Đối đỉnh",
        _ => kind(angle).0,
    }
}
pub fn point_name(id: &str, vi: bool) -> &str {
    match (id, vi) {
        ("sun", true) => "Mặt Trời",
        ("moon", true) => "Mặt Trăng",
        ("mercury", true) => "Sao Thủy",
        ("venus", true) => "Sao Kim",
        ("mars", true) => "Sao Hỏa",
        ("jupiter", true) => "Sao Mộc",
        ("saturn", true) => "Sao Thổ",
        ("uranus", true) => "Thiên Vương",
        ("neptune", true) => "Hải Vương",
        ("pluto", true) => "Diêm Vương",
        ("sun", false) => "Sun",
        ("moon", false) => "Moon",
        ("mercury", false) => "Mercury",
        ("venus", false) => "Venus",
        ("mars", false) => "Mars",
        ("jupiter", false) => "Jupiter",
        ("saturn", false) => "Saturn",
        ("uranus", false) => "Uranus",
        ("neptune", false) => "Neptune",
        ("pluto", false) => "Pluto",
        ("chiron", _) => "Chiron",
        ("trueNorthNode", _) => "North Node (true)",
        ("trueSouthNode", _) => "South Node (true)",
        ("meanNorthNode", _) => "North Node (mean)",
        ("meanSouthNode", _) => "South Node (mean)",
        ("meanLilith", _) => "Lilith (mean)",
        ("osculatingLilith", _) => "Lilith (osculating)",
        _ => id,
    }
}
