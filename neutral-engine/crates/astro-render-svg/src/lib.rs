//! Standalone vector chart rendering; never invokes the astronomical provider.
mod model;
mod panels;
mod scene;
mod styles;
mod symbols;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use symbols::{Font, Symbol, SIGNS};

pub const RENDERER_VERSION: &str = "1.2.0";
#[derive(Debug)]
pub struct RenderError {
    pub code: &'static str,
    pub message: String,
}
fn error(code: &'static str, message: impl Into<String>) -> RenderError {
    RenderError {
        code,
        message: message.into(),
    }
}
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct Options {
    sections: Option<Vec<String>>,
    output: Option<String>,
    focus: Option<model::Focus>,
    detail: Option<String>,
    theme: Option<String>,
    size: Option<u32>,
    locale: Option<String>,
    title: Option<String>,
    chart: Option<String>,
    instance_id: Option<String>,
    orientation: Option<String>,
    max_aspect_lines: Option<usize>,
    aspect_angles: Option<Vec<f64>>,
    body_ids: Option<Vec<String>>,
    show_aspects: Option<bool>,
    aspect_colors: BTreeMap<String, String>,
    aspect_groups: Option<Vec<String>>,
    aspect_endpoints: Option<String>,
    max_orb: Option<f64>,
    aspect_phase: Option<String>,
    display: Display,
    max_aspect_rows: Option<usize>,
    zodiac_symbols: BTreeMap<String, Symbol>,
    planet_symbols: BTreeMap<String, Symbol>,
}
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
struct Display {
    degree_ticks: Option<bool>,
    planet_degrees: Option<bool>,
    houses: Option<bool>,
    angles: Option<bool>,
    positions_table: Option<bool>,
    houses_table: Option<bool>,
    aspect_table: Option<bool>,
    aspect_matrix: Option<bool>,
    legend: Option<bool>,
}
struct Paint {
    bg: &'static str,
    panel: &'static str,
    line: &'static str,
    text: &'static str,
    muted: &'static str,
    blue: &'static str,
    violet: &'static str,
    peach: &'static str,
}
fn paint(light: bool) -> Paint {
    if light {
        Paint {
            bg: "#F8F9FD",
            panel: "#FFFFFF",
            line: "#BCC8DE",
            text: "#131A30",
            muted: "#5F6D89",
            blue: "#145BFF",
            violet: "#7745D9",
            peach: "#AB5739",
        }
    } else {
        Paint {
            bg: "#0B1023",
            panel: "#10182E",
            line: "#435272",
            text: "#F1F4FF",
            muted: "#A2AEC8",
            blue: "#74ADFF",
            violet: "#B09AFF",
            peach: "#FFCAA8",
        }
    }
}
fn pos(longitude: f64, r: f64, asc: f64) -> (f64, f64) {
    let a = (180. + asc - longitude).to_radians();
    (500. + r * a.cos(), 520. + r * a.sin())
}
fn line(s: &mut String, a: (f64, f64), b: (f64, f64), color: &str, width: f64, extra: &str) {
    write!(s,"<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{color}\" stroke-width=\"{width}\" {extra}/>",a.0,a.1,b.0,b.1).unwrap();
}
fn circle(s: &mut String, r: f64, color: &str, width: f64) {
    write!(s,"<circle cx=\"500\" cy=\"520\" r=\"{r}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{width}\"/>").unwrap();
}
fn zodiac_sector(s: &mut String, start: f64, asc: f64, color: &str) {
    let a = pos(start, 352., asc);
    let b = pos(start + 30., 352., asc);
    let c = pos(start + 30., 308., asc);
    let d = pos(start, 308., asc);
    write!(s, "<path d=\"M{} {} A352 352 0 0 0 {} {} L{} {} A308 308 0 0 1 {} {} Z\" fill=\"{color}\" opacity=\"0.09\"/>", a.0, a.1, b.0, b.1, c.0, c.1, d.0, d.1).unwrap();
}
fn body_color<'a>(body: &str, secondary: bool, p: &'a Paint) -> &'a str {
    if secondary {
        p.violet
    } else {
        match body {
            "sun" | "moon" => p.peach,
            "uranus" | "neptune" | "pluto" => p.violet,
            _ => p.blue,
        }
    }
}
/// DMS labels truncate within the source sign. Source coordinates are never quantized.
fn dms(longitude: f64) -> String {
    let n = ((longitude.rem_euclid(360.) % 30.) * 3600.).floor() as u64;
    format!("{:02}° {:02}′ {:02}″", n / 3600, (n / 60) % 60, n % 60)
}
fn label_positions(points: &[&scene::Point], gap: f64) -> BTreeMap<String, f64> {
    let mut sorted: Vec<_> = points.iter().map(|p| (p.id.clone(), p.longitude)).collect();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    if sorted.len() < 2 {
        return sorted.into_iter().collect();
    }
    let start = (0..sorted.len())
        .max_by(|&a, &b| {
            let gap_a = (sorted[(a + 1) % sorted.len()].1 - sorted[a].1).rem_euclid(360.);
            let gap_b = (sorted[(b + 1) % sorted.len()].1 - sorted[b].1).rem_euclid(360.);
            gap_a.total_cmp(&gap_b).then(a.cmp(&b))
        })
        .map(|i| (i + 1) % sorted.len())
        .unwrap();
    sorted.rotate_left(start);
    // Isotonic regression minimizes squared displacement. Only crowded blocks
    // move; unrelated labels stay at their original longitude.
    let mut blocks: Vec<(usize, usize, f64)> = Vec::new();
    let mut last_original = sorted[0].1;
    for (i, (_, raw)) in sorted.iter().enumerate() {
        let original = if last_original >= 360. || (i > 0 && *raw < last_original.rem_euclid(360.))
        {
            *raw + 360.
        } else {
            *raw
        };
        blocks.push((i, 1, original - i as f64 * gap));
        while blocks.len() > 1 {
            let b = blocks[blocks.len() - 1];
            let a = blocks[blocks.len() - 2];
            if a.2 / a.1 as f64 <= b.2 / b.1 as f64 {
                break;
            }
            blocks.pop();
            blocks.pop();
            blocks.push((a.0, a.1 + b.1, a.2 + b.2));
        }
        last_original = original;
    }
    let mut out = BTreeMap::new();
    for (start, count, sum) in blocks {
        for (i, (id, _)) in sorted.iter().enumerate().skip(start).take(count) {
            out.insert(
                id.clone(),
                (sum / count as f64 + i as f64 * gap).rem_euclid(360.),
            );
        }
    }
    out
}

/// Render a supported SDK result envelope without provider calls, DOM, CSS or font resources.
pub fn render(payload: &Value, options: Options) -> Result<Value, RenderError> {
    let detail = options.detail.as_deref().unwrap_or("basic");
    let theme = options.theme.as_deref().unwrap_or("dark");
    let locale = options.locale.as_deref().unwrap_or("vi");
    let orientation = options.orientation.as_deref().unwrap_or("ascendant");
    let size = options.size.unwrap_or(1000);
    if !["basic", "advanced", "professional"].contains(&detail)
        || !["dark", "light"].contains(&theme)
        || !["vi", "en"].contains(&locale)
        || !["ascendant", "aries"].contains(&orientation)
        || !(128..=4096).contains(&size)
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "Invalid detail/theme/locale/orientation/size (128..4096)",
        ));
    }
    if options.max_aspect_lines.is_some_and(|n| n > 4096)
        || options.aspect_angles.as_ref().is_some_and(|a| {
            a.len() > 32
                || a.iter()
                    .any(|n| !n.is_finite() || !(0.0..=180.).contains(n))
        })
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "Invalid aspect filter or line limit",
        ));
    }
    styles::validate(&options.aspect_colors)?;
    if options.aspect_groups.as_ref().is_some_and(|groups| {
        groups.len() > 5
            || groups.iter().any(|g| {
                !["harmonious", "challenging", "neutral", "minor", "other"].contains(&g.as_str())
            })
    }) || options
        .max_orb
        .is_some_and(|n| !n.is_finite() || !(0.0..=180.).contains(&n))
        || options
            .aspect_phase
            .as_ref()
            .is_some_and(|p| !["applying", "separating", "unknown"].contains(&p.as_str()))
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "Invalid aspectGroups/maxOrb/aspectPhase",
        ));
    }
    if options.max_aspect_rows.is_some_and(|n| n > 4096) {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "maxAspectRows must be 0..4096",
        ));
    }
    let prefix = options.instance_id.as_deref().unwrap_or("astro-chart");
    if prefix.is_empty()
        || prefix.len() > 64
        || !prefix
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
        || !prefix.as_bytes()[0].is_ascii_alphabetic()
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "instanceId must start with a letter and contain only ASCII letters, digits, _ or -",
        ));
    }
    if options
        .zodiac_symbols
        .keys()
        .any(|k| !SIGNS.contains(&k.as_str()))
        || options
            .planet_symbols
            .keys()
            .any(|k| !symbols::BODIES.contains(&k.as_str()))
    {
        return Err(error("INVALID_SYMBOL", "Wrong symbol group"));
    }
    symbols::validate(&options.zodiac_symbols)?;
    symbols::validate(&options.planet_symbols)?;
    if options
        .output
        .as_ref()
        .is_some_and(|o| !["svg", "model"].contains(&o.as_str()))
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "output must be svg or model",
        ));
    }
    if let Some(parts) = &options.sections {
        let mut seen = BTreeSet::new();
        if parts.is_empty()
            || parts.len() > 6
            || parts
                .iter()
                .any(|p| !model::PARTS.contains(&p.as_str()) || !seen.insert(p))
        {
            return Err(error(
                "INVALID_RENDER_OPTIONS",
                "sections requires 1..6 unique supported parts",
            ));
        }
    }
    let scene = scene::parse(payload, options.chart.as_deref())?;
    let house_data = model::houses(payload, &scene)?;
    let (focus, selected) = model::focus(&options, &scene, &house_data)?;
    let endpoints = options
        .aspect_endpoints
        .as_deref()
        .unwrap_or(if options.focus.is_some() {
            "all"
        } else {
            "bodies"
        });
    if !["bodies", "bodiesAndAngles", "all"].contains(&endpoints) {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "aspectEndpoints must be bodies, bodiesAndAngles or all",
        ));
    }
    let anchors = if endpoints == "bodies" {
        vec![]
    } else {
        model::anchors(&scene)
            .into_iter()
            .filter(|p| endpoints == "all" || !p.body.starts_with('H'))
            .collect()
    };
    let mut body_filter = BTreeSet::new();
    if let Some(ids) = &options.body_ids {
        for id in ids {
            if !symbols::BODIES.contains(&id.as_str()) || !body_filter.insert(id.as_str()) {
                return Err(error(
                    "INVALID_RENDER_OPTIONS",
                    "Unknown or duplicate bodyIds",
                ));
            }
        }
        if ids.is_empty() {
            return Err(error("INVALID_RENDER_OPTIONS", "bodyIds cannot be empty"));
        }
    }
    let visible = |p: &scene::Point| {
        options
            .body_ids
            .as_ref()
            .is_none_or(|ids| ids.contains(&p.body))
    };
    let title = options.title.as_deref().unwrap_or(if locale == "vi" {
        "Bản đồ chiêm tinh"
    } else {
        "Astrological chart"
    });
    let font = Font::get();
    if title.chars().count() > 100
        || title.chars().any(|c| c.is_control())
        || !font.supported(title)
    {
        return Err(error(
            "UNSUPPORTED_LABEL",
            "Title must be <=100 supported Latin/Vietnamese characters without controls",
        ));
    }
    let p = paint(theme == "light");
    let advanced = detail != "basic";
    let professional = detail == "professional";
    let positions_table = options.display.positions_table.unwrap_or(advanced);
    let houses_table = options.display.houses_table.unwrap_or(professional);
    let aspect_table = options.display.aspect_table.unwrap_or(professional);
    let aspect_matrix = options.display.aspect_matrix.unwrap_or(professional);
    let mut all_points: BTreeMap<_, _> = scene
        .layers
        .iter()
        .flat_map(|l| &l.points)
        .filter(|p| visible(p))
        .map(|p| (p.id.clone(), p))
        .collect();
    for point in &anchors {
        all_points.insert(point.id.clone(), point);
    }
    let mut excluded = BTreeMap::<&str, usize>::new();
    let mut rendered_aspects = vec![];
    for aspect in &scene.aspects {
        let endpoints = scene::endpoint(aspect, true, &scene.key)
            .is_some_and(|id| all_points.contains_key(&id))
            && scene::endpoint(aspect, false, &scene.key)
                .is_some_and(|id| all_points.contains_key(&id));
        let reason = if options.show_aspects == Some(false) {
            Some("disabled")
        } else if !endpoints {
            Some("endpointNotRendered")
        } else if !selected.is_empty()
            && !scene::endpoint(aspect, true, &scene.key).is_some_and(|id| selected.contains(&id))
            && !scene::endpoint(aspect, false, &scene.key).is_some_and(|id| selected.contains(&id))
        {
            Some("focusFilter")
        } else if options
            .aspect_angles
            .as_ref()
            .is_some_and(|a| !a.contains(&aspect["angle"].as_f64().unwrap()))
        {
            Some("angleFilter")
        } else if options.aspect_groups.as_ref().is_some_and(|groups| {
            !groups
                .iter()
                .any(|g| g == styles::kind(aspect["angle"].as_f64().unwrap()).1)
        }) {
            Some("groupFilter")
        } else if options.max_orb.is_some_and(|limit| {
            aspect["orb"]
                .as_f64()
                .is_none_or(|n| !n.is_finite() || n > limit)
        }) {
            Some("orbFilter")
        } else if options
            .aspect_phase
            .as_ref()
            .is_some_and(|phase| match phase.as_str() {
                "applying" => aspect["applying"] != true,
                "separating" => aspect["applying"] != false,
                _ => !aspect["applying"].is_null(),
            })
        {
            Some("phaseFilter")
        } else if options
            .max_aspect_lines
            .is_some_and(|n| rendered_aspects.len() >= n)
        {
            Some("lineLimit")
        } else {
            None
        };
        if let Some(reason) = reason {
            *excluded.entry(reason).or_default() += 1;
        } else {
            rendered_aspects.push(aspect.clone());
        }
    }
    let mut related = selected.clone();
    if !selected.is_empty() {
        for aspect in &rendered_aspects {
            for first in [true, false] {
                if let Some(id) = scene::endpoint(aspect, first, &scene.key) {
                    related.insert(id);
                }
            }
        }
    }
    let visible = |p: &scene::Point| model::visible(p, &options, &related);
    let anchor_ids: BTreeSet<_> = anchors.iter().map(|p| p.id.as_str()).collect();
    all_points.retain(|id, p| {
        if anchor_ids.contains(id.as_str()) {
            selected.is_empty() || related.contains(id)
        } else {
            visible(p)
        }
    });
    let drawn = rendered_aspects.len();
    let mut legend_angles: Vec<_> = rendered_aspects
        .iter()
        .map(|a| a["angle"].as_f64().unwrap())
        .collect();
    legend_angles.sort_by(f64::total_cmp);
    legend_angles.dedup();
    let legend_extra = if options.display.legend.unwrap_or(true) {
        (legend_angles.len().div_ceil(6).saturating_sub(2) * 28) as f64
    } else {
        0.
    };
    let table_rows = rendered_aspects
        .len()
        .min(options.max_aspect_rows.unwrap_or(256));
    let point_count = scene
        .layers
        .iter()
        .flat_map(|l| &l.points)
        .filter(|p| visible(p))
        .count();
    let mut parts = options.sections.clone().unwrap_or_else(|| {
        let mut p = vec!["wheel".into()];
        if positions_table {
            p.push("positions".into());
        }
        if houses_table {
            p.push("houses".into());
        }
        if aspect_matrix {
            p.push("aspectMatrix".into());
        }
        if aspect_table {
            p.push("aspects".into());
        }
        p
    });
    // sections is the explicit composition. Display overrides still govern wheel layers.
    if parts.iter().any(|p| p == "houseFocus") && focus["kind"] != "house" {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "houseFocus section requires focus.house",
        ));
    }
    let matrix_count = all_points.len();
    let mut sections = vec![];
    let mut cursor = 164.;
    for part in &mut parts {
        let h = match part.as_str() {
            "wheel" => 938. + legend_extra,
            "positions" => 88. + point_count as f64 * 35.,
            "houses" => {
                130. + house_data.len() as f64 * 86.
                    + scene.layers.iter().map(|l| l.angles.len()).sum::<usize>() as f64 * 28.
            }
            "aspectMatrix" => {
                100. + matrix_count as f64 * (680. / matrix_count.max(1) as f64).min(38.)
            }
            "aspects" => 120. + table_rows as f64 * 29.,
            "houseFocus" => 320.,
            _ => unreachable!(),
        };
        sections.push(json!({"part":part,"x":0,"y":cursor,"width":1000,"height":h}));
        cursor += h;
    }
    let height = cursor + 64.;
    let asc = if orientation == "aries" {
        0.
    } else {
        scene.layers[0]
            .angles
            .iter()
            .find(|(id, _)| id == "ascendant")
            .map(|(_, n)| *n)
            .ok_or_else(|| {
                error(
                    "MISSING_CHART_DATA",
                    "Ascendant orientation requires an ascendant angle",
                )
            })?
    };
    let description = if locale == "vi" {
        "Điểm đánh dấu dùng kinh độ gốc của SDK. Nhãn có thể giãn; đường nối chỉ về vị trí thật. Số độ/phút/giây hiển thị được cắt xuống; dữ liệu gốc được giữ trong metadata."
    } else {
        "Markers use original SDK longitudes. Labels may move with leader lines. Display DMS is truncated; full source coordinates remain in metadata."
    };
    let mut metadata = json!({"rendererVersion":RENDERER_VERSION,"sourceEngineVersion":payload["engineVersion"],"sourceSchemaVersion":payload["schemaVersion"],"sourceCalculation":payload["calculation"],"chart":scene.key,"orientation":orientation,"coordinatePolicy":"source f64, no angular quantization","labelPolicy":"truncate DMS within source sign","geometry":scene::source_geometry(&scene),"zodiacPack":"7mlabs-zodiac/0.5.0-draft","font":"Plus Jakarta Sans Medium, outlined","detail":detail,"aspectColors":options.aspect_colors,"aspectStylePolicy":"specific type > group > distinct theme default","sections":{"positionsTable":positions_table,"housesTable":houses_table,"aspectMatrix":aspect_matrix,"aspectTable":aspect_table}});
    metadata["sections"] = json!(sections);
    let model = model::Builder {
        scene: &scene,
        options: &options,
        houses: &house_data,
        focus: &focus,
        selected: &selected,
        visible_ids: &related,
        aspects: &rendered_aspects,
        sections: &sections,
    }
    .build(height, asc);
    let mut warnings = payload["warnings"].as_array().cloned().unwrap_or_default();
    if !excluded.is_empty() {
        warnings.push(json!("Some source aspects are filtered; inspect coverage.excluded. The source payload is unchanged."));
    }
    if parts.iter().any(|p| p == "aspects") && table_rows < drawn {
        warnings.push(json!(
            "Aspect table row limit reached; inspect coverage.aspectTableOmitted."
        ));
    }
    if !house_data.iter().all(|h| h["rulershipAvailable"] == true) {
        warnings.push(json!("Source does not include rulership for every house. Calculate natalDomains for house rulers and house focus."));
    }
    if scene.layers.len() > 1 {
        warnings.push(json!("Wheel house grid and angle labels use the first layer; focus relations follow this chart's source aspects."));
    }
    let has_aspects = parts.iter().any(|p| p == "aspects");
    let coverage = json!({"availableAspects":scene.aspects.len(),"renderedAspects":drawn,"excluded":excluded,"renderedAspectData":rendered_aspects,"availableBodies":scene.layers.iter().map(|l|l.points.len()).sum::<usize>(),"renderedBodies":point_count,"shiftedLabels":model["shiftedLabels"],"aspectTableRows":if has_aspects{table_rows}else{0},"aspectTableOmitted":if has_aspects{drawn-table_rows}else{0},"layers":scene.layers.iter().map(|l|&l.key).collect::<Vec<_>>()});
    if options.output.as_deref() == Some("model") {
        return Ok(
            json!({"model":model,"metadata":metadata,"warnings":warnings,"coverage":coverage}),
        );
    }
    let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{size}\" height=\"{}\" viewBox=\"0 0 1000 {height}\" role=\"img\" aria-labelledby=\"{prefix}-title {prefix}-desc\"><title id=\"{prefix}-title\">{}</title><desc id=\"{prefix}-desc\">{}</desc><metadata>{}</metadata><rect width=\"1000\" height=\"{height}\" rx=\"24\" fill=\"{}\"/>",size as f64*height/1000.,escape(title),escape(description),escape(&metadata.to_string()),p.bg);
    svg.push_str(&font.text("7mlabs / Astro", 64., 61., 17., p.muted, "start"));
    let title_size = 32. * (872. / font.width(title, 32.).max(1.)).min(1.);
    svg.push_str(&font.text(title, 64., 110., title_size, p.text, "start"));
    svg.push_str(&font.text(
        &format!("{} · {}", scene.key.to_uppercase(), scene.house_system),
        936.,
        61.,
        13.,
        p.muted,
        "end",
    ));
    line(&mut svg, (64., 137.), (936., 137.), p.line, 1., "");
    let panels = panels::Panels {
        font,
        paint: &p,
        options: &options,
    };
    let wheel = Wheel {
        scene: &scene,
        options: &options,
        paint: &p,
        font,
        points: &all_points,
        aspects: &rendered_aspects,
        legend_angles: &legend_angles,
        asc,
        legend_extra,
        focus: &focus,
        selected: &selected,
        visible_ids: &related,
    };
    for section in &sections {
        let part = section["part"].as_str().unwrap();
        let mut y = section["y"].as_f64().unwrap();
        write!(svg, "<g data-part=\"{part}\">").unwrap();
        match part {
            "wheel" => {
                write!(svg, "<g transform=\"translate(0 {})\">", y - 164.).unwrap();
                wheel.draw(&mut svg);
                svg.push_str("</g>");
            }
            "positions" => panels.positions(&mut svg, &scene, &related, &mut y),
            "houses" => panels.house_details(&mut svg, &scene, &house_data, &mut y),
            "aspectMatrix" => {
                panels.matrix(&mut svg, &scene, &all_points, &rendered_aspects, &mut y)
            }
            "aspects" => panels.aspects(&mut svg, &scene, &rendered_aspects, table_rows, &mut y),
            "houseFocus" => panels.house_focus(&mut svg, &focus, drawn, &mut y),
            _ => unreachable!(),
        }
        svg.push_str("</g>");
    }
    svg.push_str(&font.text(
        "7mlabs · Astro SDK",
        64.,
        height - 25.,
        11.,
        p.muted,
        "start",
    ));
    svg.push_str(&font.text(
        if locale == "vi" {
            "Tọa độ gốc · SVG vector"
        } else {
            "Source coordinates · Vector SVG"
        },
        936.,
        height - 25.,
        11.,
        p.muted,
        "end",
    ));
    svg.push_str("</svg>");
    Ok(json!({"svg":svg,"model":model,"metadata":metadata,"warnings":warnings,"coverage":coverage}))
}

struct Wheel<'a> {
    scene: &'a scene::Scene,
    options: &'a Options,
    paint: &'a Paint,
    font: &'a Font,
    points: &'a BTreeMap<String, &'a scene::Point>,
    aspects: &'a [Value],
    legend_angles: &'a [f64],
    asc: f64,
    legend_extra: f64,
    focus: &'a Value,
    selected: &'a BTreeSet<String>,
    visible_ids: &'a BTreeSet<String>,
}
impl Wheel<'_> {
    fn draw(&self, svg: &mut String) -> usize {
        let Self {
            scene,
            options,
            paint: p,
            font,
            points: all_points,
            aspects: rendered_aspects,
            legend_angles,
            asc,
            legend_extra,
            focus,
            selected,
            visible_ids,
        } = self;
        let (asc, legend_extra) = (*asc, *legend_extra);
        let theme = options.theme.as_deref().unwrap_or("dark");
        let locale = options.locale.as_deref().unwrap_or("vi");
        let advanced = options.detail.as_deref().unwrap_or("basic") != "basic";
        let drawn = rendered_aspects.len();
        let visible = |p: &scene::Point| model::visible(p, options, visible_ids);
        // This is a presentation transform only. Every point, cusp and aspect still
        // uses the same source longitude and the same orientation function.
        svg.push_str("<g data-chart-wheel=\"true\" transform=\"translate(500 565) scale(1.08) translate(-500 -520)\">");
        write!(
            svg,
            "<circle cx=\"500\" cy=\"520\" r=\"352\" fill=\"{}\"/>",
            p.panel
        )
        .unwrap();
        for i in 0..12 {
            zodiac_sector(
                svg,
                i as f64 * 30.,
                asc,
                [p.peach, p.violet, p.blue, p.text][i % 4],
            );
        }
        for (i, r) in [352., 308., 222., 178.].iter().enumerate() {
            circle(svg, *r, p.line, if i == 0 { 2. } else { 1.2 });
        }
        if options.display.degree_ticks.unwrap_or(true) {
            for degree in 0..360 {
                let outer = 308.;
                let inner = if degree % 30 == 0 {
                    293.
                } else if degree % 10 == 0 {
                    298.
                } else if degree % 5 == 0 {
                    301.
                } else {
                    304.
                };
                line(
                    svg,
                    pos(degree as f64, inner, asc),
                    pos(degree as f64, outer, asc),
                    p.muted,
                    if degree % 30 == 0 {
                        1.6
                    } else if degree % 5 == 0 {
                        1.
                    } else {
                        0.65
                    },
                    "",
                );
            }
        }
        for (i, sign) in SIGNS.iter().enumerate() {
            line(
                svg,
                pos(i as f64 * 30., 308., asc),
                pos(i as f64 * 30., 352., asc),
                p.line,
                1.,
                "",
            );
            let (x, y) = pos(i as f64 * 30. + 15., 330., asc);
            let color = [p.peach, p.violet, p.blue, p.text][i % 4];
            svg.push_str(&symbols::draw(
                sign,
                x,
                y,
                35.,
                color,
                &options.zodiac_symbols,
            ));
        }
        if let Some(h) = focus.get("house").filter(|h| h.is_object()) {
            let start = h["longitude"].as_f64().unwrap();
            let width = h["width"].as_f64().unwrap();
            let a = pos(start, 222., asc);
            let b = pos(start + width, 222., asc);
            let c = pos(start + width, 178., asc);
            let d = pos(start, 178., asc);
            write!(svg,"<path data-focus-house=\"{}\" d=\"M{} {} A222 222 0 {} 0 {} {} L{} {} A178 178 0 {} 1 {} {} Z\" fill=\"{}\" opacity=\"0.22\"/>",h["number"],a.0,a.1,if width>180.{1}else{0},b.0,b.1,c.0,c.1,if width>180.{1}else{0},d.0,d.1,p.violet).unwrap();
        }
        if options.display.houses.unwrap_or(true) {
            for (index, layer) in scene.layers.iter().enumerate() {
                for (i, cusp) in layer.houses.iter().enumerate() {
                    let (inner, outer) = if index == 0 {
                        (178., 308.)
                    } else {
                        (185., 222.)
                    };
                    line(
                        svg,
                        pos(*cusp, inner, asc),
                        pos(*cusp, outer, asc),
                        if index == 0 { p.line } else { p.violet },
                        0.9,
                        if index == 0 {
                            ""
                        } else {
                            "stroke-dasharray=\"4 5\""
                        },
                    );
                    if index == 0 {
                        let center =
                            cusp + (layer.houses[(i + 1) % 12] - cusp).rem_euclid(360.) / 2.;
                        let (x, y) = pos(center, 195., asc);
                        svg.push_str(&font.text(
                            &(i + 1).to_string(),
                            x,
                            y + 5.,
                            15.,
                            p.muted,
                            "middle",
                        ));
                    }
                }
            }
        }
        for aspect in rendered_aspects.iter() {
            let a = all_points[&scene::endpoint(aspect, true, &scene.key).unwrap()];
            let b = all_points[&scene::endpoint(aspect, false, &scene.key).unwrap()];
            let angle = aspect["angle"].as_f64().unwrap();
            let color = styles::color(angle, theme == "light", &options.aspect_colors);
            write!(svg,"<g data-aspect-type=\"{}\" data-aspect-angle=\"{angle}\"><title>{} / {}: {angle}°</title>", styles::kind(angle).0, escape(&a.id),escape(&b.id)).unwrap();
            line(
                svg,
                pos(a.longitude, 176., asc),
                pos(b.longitude, 176., asc),
                color,
                if angle == 180. { 2. } else { 1.5 },
                &format!("opacity=\"0.82\" {}", styles::line_attributes(angle)),
            );
            svg.push_str("</g>");
        }
        let mut shifted = 0;
        for (index, layer) in scene.layers.iter().enumerate() {
            let points: Vec<_> = layer.points.iter().filter(|p| visible(p)).collect();
            let radius = if index == 0 { 275. } else { 239. };
            let labels = label_positions(&points, if index == 0 { 9. } else { 10.5 });
            for point in points {
                let dim = !selected.is_empty() && !selected.contains(&point.id);
                if dim {
                    write!(
                        svg,
                        "<g opacity=\"{}\">",
                        if visible_ids.contains(&point.id) {
                            0.65
                        } else {
                            0.25
                        }
                    )
                    .unwrap();
                }
                let color = body_color(&point.body, index > 0, p);
                let real_radius = if index == 0 { 299. } else { 223. };
                let real = pos(point.longitude, real_radius, asc);
                let label = pos(labels[&point.id], radius, asc);
                let delta = (labels[&point.id] - point.longitude + 180.).rem_euclid(360.) - 180.;
                if delta.abs() > 0.05 {
                    shifted += 1;
                }
                line(svg, real, label, color, 0.9, "opacity=\"0.7\"");
                if index == 0 {
                    line(
                        svg,
                        pos(point.longitude, 302., asc),
                        pos(point.longitude, 308., asc),
                        color,
                        1.8,
                        "",
                    );
                }
                write!(svg,"<circle cx=\"{}\" cy=\"{}\" r=\"2.7\" fill=\"{color}\" data-point-id=\"{}\" data-longitude=\"{}\"/>",real.0,real.1,escape(&point.id),point.longitude).unwrap();
                write!(
                    svg,
                    "<g data-label-for=\"{}\"><title>{}: {}°</title>",
                    escape(&point.id),
                    escape(&point.id),
                    point.longitude
                )
                .unwrap();
                write!(
                    svg,
                    "<circle cx=\"{}\" cy=\"{}\" r=\"17\" fill=\"{}\"/>",
                    label.0, label.1, p.panel
                )
                .unwrap();
                svg.push_str(&symbols::draw(
                    &point.body,
                    label.0,
                    label.1,
                    30.,
                    color,
                    &options.planet_symbols,
                ));
                svg.push_str("</g>");
                if options.display.planet_degrees.unwrap_or(advanced) && scene.layers.len() == 1 {
                    let dm = dms(point.longitude)
                        .split_whitespace()
                        .take(2)
                        .collect::<String>();
                    let (x, y) = pos(labels[&point.id], 250., asc);
                    svg.push_str(&font.text(&dm, x, y + 3., 10., p.text, "middle"));
                }
                if point.retrograde == Some(true) {
                    svg.push_str(&font.text(
                        "R",
                        label.0 + 14.,
                        label.1 + 15.,
                        10.,
                        p.peach,
                        "start",
                    ));
                }
                if dim {
                    svg.push_str("</g>");
                }
            }
        }
        if options.display.angles.unwrap_or(true) {
            for (id, n) in &scene.layers[0].angles {
                let name = match id.as_str() {
                    "ascendant" => "ASC",
                    "midheaven" => "MC",
                    "descendant" => "DSC",
                    _ => "IC",
                };
                let color = if ["ascendant", "descendant"].contains(&id.as_str()) {
                    p.blue
                } else {
                    p.peach
                };
                line(svg, pos(*n, 220., asc), pos(*n, 364., asc), color, 1.8, "");
                let (x, y) = pos(*n, 379., asc);
                svg.push_str(&font.text(name, x, y + 5., 14., color, "middle"));
            }
        }
        svg.push_str("</g>");
        if drawn > 0 && options.display.legend.unwrap_or(true) {
            for (i, angle) in legend_angles.iter().enumerate() {
                let x = 80. + (i % 6) as f64 * 148.;
                let y = 1021. + (i / 6) as f64 * 28.;
                let color = styles::color(*angle, theme == "light", &options.aspect_colors);
                line(
                    svg,
                    (x, y),
                    (x + 27., y),
                    color,
                    2.,
                    styles::line_attributes(*angle),
                );
                svg.push_str(&font.text(
                    &format!("{angle}°"),
                    x + 36.,
                    y + 4.,
                    12.,
                    p.muted,
                    "start",
                ));
            }
        }
        svg.push_str(&font.text(
            &format!(
                "{} / {} {}",
                drawn,
                scene.aspects.len(),
                if locale == "vi" {
                    "góc chiếu được vẽ"
                } else {
                    "aspects rendered"
                }
            ),
            500.,
            1083. + legend_extra,
            13.,
            p.muted,
            "middle",
        ));
        shifted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn natal() -> Value {
        serde_json::from_str(include_str!("../../../../examples/natal-result.json")).unwrap()
    }
    #[test]
    fn standalone_font_free_deterministic_and_original_coordinates() {
        let input = natal();
        let before = input.clone();
        let out = render(&input, Options::default()).unwrap();
        assert_eq!(out, render(&input, Options::default()).unwrap());
        assert_eq!(input, before);
        let svg = out["svg"].as_str().unwrap();
        for forbidden in [
            "<text",
            "<script",
            "href=",
            "var(--",
            "font-family",
            "<image",
        ] {
            assert!(!svg.contains(forbidden), "{forbidden}");
        }
        assert!(svg.contains("data-longitude=\"280.368"));
        assert_eq!(out["coverage"]["renderedBodies"], 10);
        assert_eq!(
            out["coverage"]["renderedAspects"],
            input["data"]["aspects"].as_array().unwrap().len()
        );
    }
    #[test]
    fn independent_parts_compose_in_order_and_model_only_has_no_svg() {
        let input = natal();
        let before = input.clone();
        for part in ["wheel", "positions", "houses", "aspectMatrix", "aspects"] {
            let o =
                serde_json::from_value(json!({"sections":[part],"detail":"professional"})).unwrap();
            let out = render(&input, o).unwrap();
            let svg = out["svg"].as_str().unwrap();
            assert_eq!(out["model"]["sections"].as_array().unwrap().len(), 1);
            assert_eq!(out["model"]["sections"][0]["part"], part);
            assert_eq!(svg.matches("data-part=").count(), 1);
            assert_eq!(svg.contains("data-chart-wheel"), part == "wheel");
            if part != "wheel" {
                assert!(out["model"]["hitTargets"].as_array().unwrap().is_empty());
                assert!(out["model"]["points"][0]["markerPosition"].is_null());
            }
        }
        let o = || {
            serde_json::from_value(json!({"sections":["positions","wheel"],"output":"model"}))
                .unwrap()
        };
        let out = render(&input, o()).unwrap();
        assert!(out.get("svg").is_none());
        assert_eq!(out, render(&input, o()).unwrap());
        assert_eq!(out["model"]["sections"][0]["part"], "positions");
        assert!(out["model"]["hitTargets"]
            .as_array()
            .unwrap()
            .iter()
            .all(|h| h["center"]["y"].as_f64().unwrap() > 164.));
        assert_eq!(input, before);
    }
    #[test]
    fn house_focus_uses_source_ruler_and_includes_cusp_contacts() {
        let input: Value = serde_json::from_str(include_str!(
            "../../../../examples/natal-domains-result.json"
        ))
        .unwrap();
        let before = input.clone();
        let source = &input["data"]["context"]["houses"][6];
        let o = serde_json::from_value(
            json!({"sections":["wheel","houseFocus","aspectMatrix"],"focus":{"house":7}}),
        )
        .unwrap();
        let out = render(&input, o).unwrap();
        assert_eq!(
            out["model"]["focus"]["house"]["rulerId"],
            source["rulerBodyId"]
        );
        assert!(out["svg"]
            .as_str()
            .unwrap()
            .contains("data-focus-house=\"7\""));
        let selected = out["model"]["selectedPointIds"].as_array().unwrap();
        assert!(selected.contains(&json!("H7")));
        for a in out["model"]["aspects"].as_array().unwrap() {
            assert!(selected.contains(&a["endpoint1"]) || selected.contains(&a["endpoint2"]));
            assert!(a["start"]["x"].is_number());
        }
        assert_eq!(
            out["coverage"]["availableAspects"].as_u64().unwrap(),
            out["coverage"]["renderedAspects"].as_u64().unwrap()
                + out["coverage"]["excluded"]
                    .as_object()
                    .unwrap()
                    .values()
                    .map(|n| n.as_u64().unwrap())
                    .sum::<u64>()
        );
        assert_eq!(input, before);
        let o = serde_json::from_value(
            json!({"sections":["positions"],"focus":{"house":7,"mode":"isolate"}}),
        )
        .unwrap();
        let out = render(&input, o).unwrap();
        let ids = out["model"]["relatedPointIds"].as_array().unwrap();
        assert!(out["model"]["points"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| ids.contains(&p["id"])));
        let o = serde_json::from_value(json!({"focus":{"house":7}})).unwrap();
        assert_eq!(
            render(&natal(), o).unwrap_err().code,
            "MISSING_HOUSE_CONTEXT"
        );
    }
    #[test]
    fn focus_preserves_layer_namespaces_and_source_rulership() {
        for (fixture, chart, layer) in [
            ("couple", "synastry", "B"),
            ("couple", "A", "A"),
            ("composite", "C", "C"),
            ("forecast-day", "transits", "N"),
        ] {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../examples");
            let p: Value = serde_json::from_str(
                &std::fs::read_to_string(root.join(format!("{fixture}-result.json"))).unwrap(),
            )
            .unwrap();
            let o=serde_json::from_value(json!({"chart":chart,"sections":["wheel","houseFocus"],"focus":{"house":7,"layer":layer},"output":"model"})).unwrap();
            let out = render(&p, o).unwrap();
            assert_eq!(out["model"]["focus"]["house"]["layer"], layer);
            if ["synastry", "transits"].contains(&chart) {
                assert!(out["model"]["focus"]["house"]["rulerId"]
                    .as_str()
                    .unwrap()
                    .starts_with(&format!("{layer}:")));
            }
        }
    }
    #[test]
    fn composition_and_focus_invalid_states_are_explicit() {
        for value in [
            json!({"sections":[]}),
            json!({"sections":["wheel","wheel"]}),
            json!({"sections":["unknown"]}),
            json!({"sections":["houseFocus"]}),
            json!({"focus":{"house":13}}),
            json!({"focus":{"pointIds":["fake"]}}),
            json!({"focus":{"house":1,"pointIds":["sun"]}}),
            json!({"output":"html"}),
        ] {
            let o = serde_json::from_value(value).unwrap();
            assert!(render(&natal(), o).is_err());
        }
        let o = serde_json::from_value(
            json!({"focus":{"pointIds":["sun"]},"sections":["wheel"],"output":"model"}),
        )
        .unwrap();
        let out = render(&natal(), o).unwrap();
        assert_eq!(out["model"]["focus"]["kind"], "points");
        assert!(out["model"]["aspects"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| a["endpoint1"] == "sun" || a["endpoint2"] == "sun"));
    }
    #[test]
    fn professional_palette_precedence_filters_matrix_and_table() {
        let mut p = natal();
        let ids = ["sun", "moon", "mercury", "venus", "mars", "jupiter"];
        p["data"]["aspects"]=json!([0.,60.,90.,120.,180.].iter().enumerate().map(|(i,a)|json!({"body1":ids[i],"body2":ids[i+1],"angle":a,"orb":1.25,"maxOrb":6.,"applying":true})).collect::<Vec<_>>());
        let options = || {
            serde_json::from_value::<Options>(json!({"detail":"professional","aspectColors":{"harmonious":"#123456","trine":"#abcdef","challenging":"#C02","neutral":"#847"},"maxAspectRows":2})).unwrap()
        };
        let out = render(&p, options()).unwrap();
        let svg = out["svg"].as_str().unwrap();
        assert!(svg.contains("data-section=\"aspect-matrix\""));
        assert!(svg.contains("data-section=\"aspect-table\""));
        for color in ["#123456", "#abcdef", "#C02", "#847"] {
            assert!(svg.contains(color));
        }
        assert_eq!(out["coverage"]["aspectTableRows"], 2);
        assert_eq!(out["coverage"]["aspectTableOmitted"], 3);
        assert_eq!(out["coverage"]["renderedAspects"], 5);
        let mut o = options();
        o.aspect_groups = Some(vec!["harmonious".into()]);
        o.max_orb = Some(2.);
        o.aspect_phase = Some("applying".into());
        let out = render(&p, o).unwrap();
        assert_eq!(out["coverage"]["renderedAspects"], 2);
        assert_eq!(out["coverage"]["excluded"]["groupFilter"], 3);
        let mut o = options();
        o.max_orb = Some(1.);
        let out = render(&p, o).unwrap();
        assert_eq!(out["coverage"]["renderedAspects"], 0);
        let mut o = options();
        o.aspect_colors
            .insert("harmonious".into(), "url(https://bad)".into());
        assert!(render(&p, o).is_err());
        let mut o = options();
        o.aspect_colors.insert("typo".into(), "#fff".into());
        assert!(render(&p, o).is_err());
        for angle in [0., 30., 45., 60., 72., 90., 120., 135., 144., 150., 180.] {
            assert_ne!(styles::color(angle, false, &BTreeMap::new()), "");
        }
        assert_ne!(
            styles::color(60., false, &BTreeMap::new()),
            styles::color(120., false, &BTreeMap::new())
        );
        assert_ne!(
            styles::color(90., false, &BTreeMap::new()),
            styles::color(180., false, &BTreeMap::new())
        );
    }
    #[test]
    fn additional_points_keep_geometry_and_support_custom_symbols() {
        let mut p = natal();
        p["data"]["additionalPoints"] = json!([{"id":"chiron","longitude":0.,"house":1,"isRetrograde":false},{"id":"trueNorthNode","longitude":359.999,"house":12,"isRetrograde":true},{"id":"meanLilith","longitude":0.001,"house":1,"isRetrograde":false}]);
        let o=serde_json::from_value(json!({"detail":"professional","bodyIds":["sun","chiron","trueNorthNode","meanLilith"],"planetSymbols":{"chiron":{"viewBox":[0,0,32,32],"nodes":[["path",{"d":"M1 1L31 31"}]]}}})).unwrap();
        let out = render(&p, o).unwrap();
        assert_eq!(out["coverage"]["renderedBodies"], 4);
        assert!(out["svg"].as_str().unwrap().contains("M1 1L31 31"));
        assert!(out["svg"]
            .as_str()
            .unwrap()
            .contains("data-longitude=\"0\""));
    }
    #[test]
    fn advanced_light_filter_and_custom_symbols() {
        let opts:Options=serde_json::from_value(json!({"detail":"advanced","theme":"light","bodyIds":["sun","moon"],"maxAspectLines":0,"zodiacSymbols":{"aries":{"viewBox":[0,0,32,32],"nodes":[["path",{"d":"M4 28L16 4L28 28"}]]}}})).unwrap();
        let out = render(&natal(), opts).unwrap();
        assert_eq!(out["coverage"]["renderedBodies"], 2);
        assert_eq!(out["coverage"]["renderedAspects"], 0);
        assert!(out["svg"].as_str().unwrap().contains("M4 28L16 4L28 28"));
    }
    #[test]
    fn invalid_payload_options_and_injection_are_errors() {
        let mut p = natal();
        p["errors"] = json!([{"code":"FAIL"}]);
        assert!(render(&p, Options::default()).is_err());
        let o: Options = serde_json::from_value(json!({"title":"bad\nlabel"})).unwrap();
        assert!(render(&natal(), o).is_err());
        let o:Options=serde_json::from_value(json!({"zodiacSymbols":{"aries":{"viewBox":[0,0,32,32],"nodes":[["script",{"d":"M0 0"}]]}}})).unwrap();
        assert!(render(&natal(), o).is_err());
        let mut p = natal();
        p["data"]["placements"][0]["longitude"] = json!(360.);
        assert!(render(&p, Options::default()).is_err());
    }
    #[test]
    fn dms_and_label_layout_across_zero() {
        assert_eq!(dms(29.9999), "29° 59′ 59″");
        assert_eq!(dms(359.9999), "29° 59′ 59″");
        assert_eq!(dms(0.), "00° 00′ 00″");
        let mut points = vec![];
        for (i, n) in [358., 359., 0., 1., 2., 3., 4., 5., 6., 7.]
            .iter()
            .enumerate()
        {
            points.push(scene::Point {
                id: i.to_string(),
                body: "sun".into(),
                longitude: *n,
                retrograde: None,
                house: None,
            });
        }
        let refs: Vec<_> = points.iter().collect();
        let out = label_positions(&refs, 10.);
        let mut a: Vec<_> = out.values().copied().collect();
        a.sort_by(f64::total_cmp);
        assert!((0..a.len()).all(|i| (a[(i + 1) % a.len()] - a[i]).rem_euclid(360.) >= 9.999));
    }
    #[test]
    fn random_dense_and_sparse_layouts_keep_a_minimum_circular_gap() {
        let mut seed = 7u64;
        for _ in 0..1000 {
            let mut points = vec![];
            for i in 0..10 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                let longitude = (seed >> 32) as f64 / u32::MAX as f64 * 359.999;
                points.push(scene::Point {
                    id: i.to_string(),
                    body: "sun".into(),
                    longitude,
                    retrograde: None,
                    house: None,
                });
            }
            let refs: Vec<_> = points.iter().collect();
            let out = label_positions(&refs, 10.5);
            let mut values: Vec<_> = out.values().copied().collect();
            values.sort_by(f64::total_cmp);
            assert!(
                (0..values.len()).all(|i| (values[(i + 1) % values.len()] - values[i])
                    .rem_euclid(360.)
                    >= 10.499999)
            );
        }
    }
    #[test]
    fn supported_chart_adapters() {
        for (raw, key, count) in [
            (
                include_str!("../../../../examples/natal-domains-result.json"),
                "natal",
                10,
            ),
            (
                include_str!("../../../../examples/couple-result.json"),
                "synastry",
                20,
            ),
            (
                include_str!("../../../../examples/composite-result.json"),
                "C",
                10,
            ),
            (
                include_str!("../../../../examples/forecast-day-result.json"),
                "transits",
                20,
            ),
        ] {
            let p = serde_json::from_str(raw).unwrap();
            let opts: Options = serde_json::from_value(json!({"chart":key})).unwrap();
            let result = render(&p, opts).unwrap();
            assert_eq!(result["coverage"]["renderedBodies"], count);
        }
    }
}
