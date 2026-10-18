//! Source-derived presentation data. No ephemeris calls or invented rulership.
use crate::{error, label_positions, pos, scene, styles, Options, RenderError};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const PARTS: [&str; 6] = [
    "wheel",
    "positions",
    "houses",
    "aspectMatrix",
    "aspects",
    "houseFocus",
];
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct Focus {
    pub house: Option<u8>,
    pub layer: Option<String>,
    pub point_ids: Option<Vec<String>>,
    pub mode: Option<String>,
}
fn qualified(scene: &scene::Scene, layer: &str, id: &str) -> String {
    if ["synastry", "transits"].contains(&scene.key.as_str()) {
        format!("{layer}:{id}")
    } else {
        id.into()
    }
}
fn context<'a>(payload: &'a Value, scene: &scene::Scene, layer: &str) -> &'a Value {
    let d = &payload["data"];
    match scene.key.as_str() {
        "A" | "B" | "synastry" => &d["subjects"][layer]["context"],
        "C" => &d["composite"]["context"],
        "transits" if layer == "N" => &d["subject"]["context"],
        "transits" => &Value::Null,
        _ => &d["context"],
    }
}
pub fn houses(payload: &Value, scene: &scene::Scene) -> Result<Vec<Value>, RenderError> {
    let mut result = vec![];
    for layer in &scene.layers {
        let c = context(payload, scene, &layer.key);
        for (i, cusp) in layer.houses.iter().enumerate() {
            let source = c["houses"]
                .as_array()
                .and_then(|rows| rows.iter().find(|h| h["number"] == i + 1));
            let ruler = source.and_then(|h| h["rulerBodyId"].as_str());
            let ruler_point = ruler.and_then(|id| layer.points.iter().find(|p| p.body == id));
            if ruler.is_some() && ruler_point.is_none() {
                return Err(error(
                    "INVALID_HOUSE_CONTEXT",
                    "Source house ruler does not identify a rendered chart body",
                ));
            }
            result.push(json!({"id":qualified(scene,&layer.key,&format!("H{}",i+1)),"layer":layer.key,"number":i+1,"longitude":cusp,
                "width":(layer.houses[(i+1)%12]-cusp).rem_euclid(360.),
                "occupantIds":layer.points.iter().filter(|p| p.house==Some((i+1) as u64)).map(|p| &p.id).collect::<Vec<_>>(),
                "rulerId":ruler_point.map(|p| &p.id),"rulerPlacement":ruler_point.map(|p|json!({"id":p.id,"longitude":p.longitude,"house":p.house})),
                "rulership":if ruler.is_some(){c["rulership"].clone()}else{Value::Null},"rulershipAvailable":ruler.is_some()}));
        }
    }
    Ok(result)
}
pub fn anchors(scene: &scene::Scene) -> Vec<scene::Point> {
    scene
        .layers
        .iter()
        .flat_map(|l| {
            l.houses
                .iter()
                .enumerate()
                .map(|(i, n)| scene::Point {
                    id: qualified(scene, &l.key, &format!("H{}", i + 1)),
                    body: format!("H{}", i + 1),
                    longitude: *n,
                    retrograde: None,
                    house: Some((i + 1) as u64),
                })
                .chain(l.angles.iter().map(|(id, n)| scene::Point {
                    id: qualified(scene, &l.key, id),
                    body: id.clone(),
                    longitude: *n,
                    retrograde: None,
                    house: None,
                }))
        })
        .collect()
}
pub fn focus(
    options: &Options,
    scene: &scene::Scene,
    houses: &[Value],
) -> Result<(Value, BTreeSet<String>), RenderError> {
    let Some(f) = &options.focus else {
        return Ok((Value::Null, BTreeSet::new()));
    };
    if f.house.is_some() == f.point_ids.is_some()
        || f.mode
            .as_ref()
            .is_some_and(|m| !["highlight", "isolate"].contains(&m.as_str()))
    {
        return Err(error(
            "INVALID_RENDER_OPTIONS",
            "focus requires exactly one of house or pointIds, mode highlight/isolate",
        ));
    }
    let mut selected = BTreeSet::new();
    let value = if let Some(number) = f.house {
        let layer = f.layer.as_deref().unwrap_or(&scene.layers[0].key);
        let house = houses
            .iter()
            .find(|h| h["number"] == number && h["layer"] == layer)
            .ok_or_else(|| error("INVALID_RENDER_OPTIONS", "Unknown focus house or layer"))?;
        if house["rulershipAvailable"] != true {
            return Err(error("MISSING_HOUSE_CONTEXT","House focus requires source rulership context; calculate natalDomains or use a subject/composite context"));
        }
        selected.insert(house["id"].as_str().unwrap().into());
        selected.insert(house["rulerId"].as_str().unwrap().into());
        for id in house["occupantIds"].as_array().unwrap() {
            selected.insert(id.as_str().unwrap().into());
        }
        json!({"kind":"house","house":house,"mode":f.mode.as_deref().unwrap_or("highlight")})
    } else {
        if f.layer.is_some() {
            return Err(error(
                "INVALID_RENDER_OPTIONS",
                "Point focus uses fully qualified IDs, not layer",
            ));
        }
        let ids = f.point_ids.as_ref().unwrap();
        if ids.is_empty() || ids.len() > 34 {
            return Err(error(
                "INVALID_RENDER_OPTIONS",
                "focus.pointIds requires 1..34 chart point IDs",
            ));
        }
        let available: BTreeSet<_> = scene
            .layers
            .iter()
            .flat_map(|l| &l.points)
            .map(|p| p.id.as_str())
            .collect();
        for id in ids {
            if !available.contains(id.as_str()) || !selected.insert(id.clone()) {
                return Err(error(
                    "INVALID_RENDER_OPTIONS",
                    "Unknown or duplicate focus.pointIds",
                ));
            }
        }
        json!({"kind":"points","pointIds":ids,"mode":f.mode.as_deref().unwrap_or("highlight")})
    };
    Ok((value, selected))
}
pub fn ordered<'a>(
    scene: &'a scene::Scene,
    points: &BTreeMap<String, &'a scene::Point>,
) -> Vec<&'a scene::Point> {
    let mut out: Vec<_> = scene
        .layers
        .iter()
        .flat_map(|l| &l.points)
        .filter(|p| points.contains_key(&p.id))
        .collect();
    for (id, p) in points {
        if !out.iter().any(|p| p.id == *id) {
            out.push(p);
        }
    }
    out
}
/// All layout values are in the final SVG viewBox. Source longitude is separate.
pub struct Builder<'a> {
    pub scene: &'a scene::Scene,
    pub options: &'a Options,
    pub houses: &'a [Value],
    pub focus: &'a Value,
    pub selected: &'a BTreeSet<String>,
    pub visible_ids: &'a BTreeSet<String>,
    pub aspects: &'a [Value],
    pub sections: &'a [Value],
}
impl Builder<'_> {
    pub fn build(self, height: f64, asc: f64) -> Value {
        let Self {
            scene,
            options,
            houses,
            focus,
            selected,
            visible_ids,
            aspects,
            sections,
        } = self;
        let wheel = sections.iter().find(|s| s["part"] == "wheel");
        let wheel_y = wheel.map(|s| s["y"].as_f64().unwrap());
        let transform = |n: f64, r: f64| {
            let (x, y) = pos(n, r, asc);
            json!({"x":500.+(x-500.)*1.08,"y":565.+(y-520.)*1.08+wheel_y.unwrap_or(164.)-164.})
        };
        let mut points = vec![];
        let mut targets = vec![];
        let mut shifted = 0;
        for (i, layer) in scene.layers.iter().enumerate() {
            let visible: Vec<_> = layer
                .points
                .iter()
                .filter(|p| visible(p, options, visible_ids))
                .collect();
            let labels = label_positions(&visible, if i == 0 { 9. } else { 10.5 });
            for p in visible {
                if ((labels[&p.id] - p.longitude + 180.).rem_euclid(360.) - 180.).abs() > 0.05 {
                    shifted += 1;
                }
                let label =
                    wheel_y.map(|_| transform(labels[&p.id], if i == 0 { 275. } else { 239. }));
                let marker =
                    wheel_y.map(|_| transform(p.longitude, if i == 0 { 299. } else { 223. }));
                points.push(json!({"id":p.id,"body":p.body,"signIndex":(p.longitude/30.).floor() as usize,"signId":crate::SIGNS[(p.longitude/30.).floor() as usize],"displayDms":crate::dms(p.longitude),"layer":layer.key,"longitude":p.longitude,"house":p.house,"isRetrograde":p.retrograde,"selected":selected.contains(&p.id),"opacity":if selected.is_empty() || selected.contains(&p.id){1.}else if visible_ids.contains(&p.id){0.65}else{0.25},"labelPosition":label,"markerPosition":marker}));
                if let Some(label) = label {
                    targets.push(
                    json!({"kind":"point","id":p.id,"layer":layer.key,"center":label,"radius":18.}),
                );
                }
            }
        }
        if wheel_y.is_some() {
            for h in houses.iter().filter(|h| h["layer"] == scene.layers[0].key) {
                targets.push(json!({"kind":"house","id":h["id"],"layer":h["layer"],"house":h["number"],"center":transform(h["longitude"].as_f64().unwrap()+h["width"].as_f64().unwrap()/2.,195.),"radius":18.}));
            }
        }
        let anchors = anchors(scene);
        let endpoints: BTreeMap<_, _> = scene
            .layers
            .iter()
            .flat_map(|l| &l.points)
            .chain(&anchors)
            .map(|p| (p.id.as_str(), p))
            .collect();
        let aspects:Vec<_>=aspects.iter().map(|a| {
            let mut a=a.clone();a["endpoint1"]=json!(scene::endpoint(&a,true,&scene.key));a["endpoint2"]=json!(scene::endpoint(&a,false,&scene.key));
            for (field,key) in [("start","endpoint1"),("end","endpoint2")] {
                a[field]=a[key].as_str().and_then(|id|endpoints.get(id)).and_then(|p|wheel_y.map(|_|transform(p.longitude,176.))).unwrap_or(Value::Null);
            }
            let angle=a["angle"].as_f64().unwrap();
            let dash:&[u8]=match angle{0.=>&[2,3],60.=>&[6,3],90.=>&[10,3],120.|180.=>&[],_=>&[2,4]};
            a["style"]=json!({"type":styles::kind(angle).0,"group":styles::kind(angle).1,"color":styles::color(angle,options.theme.as_deref()==Some("light"),&options.aspect_colors),"width":if angle==180.{2.}else{1.5},"opacity":0.82,"dash":dash});a
        }).collect();
        json!({"modelVersion":"1.0","chart":scene.key,"viewBox":[0,0,1000,height],"sections":sections,"points":points,"houses":houses,"aspects":aspects,"focus":focus,"selectedPointIds":selected,"relatedPointIds":visible_ids,"hitTargets":targets,"shiftedLabels":shifted,"angles":scene.layers.iter().flat_map(|l|l.angles.iter().map(|(id,n)|json!({"id":qualified(scene,&l.key,id),"layer":l.key,"longitude":n}))).collect::<Vec<_>>(),"houseContextAvailable":houses.iter().all(|h|h["rulershipAvailable"]==true)})
    }
}

pub fn visible(point: &scene::Point, options: &Options, selected: &BTreeSet<String>) -> bool {
    if options
        .body_ids
        .as_ref()
        .is_some_and(|ids| !ids.contains(&point.body))
    {
        return false;
    }
    options
        .focus
        .as_ref()
        .is_none_or(|f| f.mode.as_deref() != Some("isolate"))
        || selected.contains(&point.id)
}
