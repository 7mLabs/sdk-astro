use crate::{body_color, dms, escape, line, scene, styles, symbols, Font, Options, Paint, SIGNS};
use serde_json::Value;
use std::{collections::BTreeMap, fmt::Write};
fn heading(svg: &mut String, title: &str, font: &Font, p: &Paint, y: f64) {
    svg.push_str(&font.text(title, 64., y + 20., 20., p.text, "start"));
    line(svg, (64., y + 38.), (936., y + 38.), p.line, 1., "");
}
pub struct Panels<'a> {
    pub font: &'a Font,
    pub paint: &'a Paint,
    pub options: &'a Options,
}
impl Panels<'_> {
    pub fn matrix(
        &self,
        svg: &mut String,
        scene: &scene::Scene,
        points: &BTreeMap<String, &scene::Point>,
        aspects: &[Value],
        y: &mut f64,
    ) {
        let (font, p, options) = (self.font, self.paint, self.options);
        let vi = options.locale.as_deref().unwrap_or("vi") == "vi";
        heading(
            svg,
            if vi {
                "Ma trận góc chiếu · theo bộ lọc"
            } else {
                "Aspect matrix · filtered"
            },
            font,
            p,
            *y,
        );
        let mut cells = BTreeMap::<(String, String), Vec<f64>>::new();
        for a in aspects {
            let first = scene::endpoint(a, true, &scene.key).unwrap();
            let second = scene::endpoint(a, false, &scene.key).unwrap();
            cells
                .entry((first.clone(), second.clone()))
                .or_default()
                .push(a["angle"].as_f64().unwrap());
            cells
                .entry((second, first))
                .or_default()
                .push(a["angle"].as_f64().unwrap());
        }
        let ordered = crate::model::ordered(scene, points);
        let cell = (680. / ordered.len().max(1) as f64).min(38.);
        *y += 85.;
        svg.push_str("<g data-section=\"aspect-matrix\">");
        for (i, point) in ordered.iter().enumerate() {
            let x = 250. + (i as f64 + 0.5) * cell;
            svg.push_str(&font.text(&(i + 1).to_string(), x, *y - 15., 11., p.muted, "middle"));
            let row = *y + i as f64 * cell;
            let name = format!("{} · {}", i + 1, point.id);
            let size = (160. / font.width(&name, 11.).max(1.)).min(1.) * 11.;
            svg.push_str(&font.text(&name, 80., row + cell * 0.65, size, p.text, "start"));
            for (j, other) in ordered.iter().enumerate() {
                let x = 250. + j as f64 * cell;
                write!(svg,"<rect x=\"{x}\" y=\"{row}\" width=\"{cell}\" height=\"{cell}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",p.panel,p.line).unwrap();
                if let Some(angles) = cells.get(&(point.id.clone(), other.id.clone())) {
                    let label = angles
                        .iter()
                        .map(|a| format!("{a}°"))
                        .collect::<Vec<_>>()
                        .join("/");
                    let color = styles::color(
                        angles[0],
                        options.theme.as_deref() == Some("light"),
                        &options.aspect_colors,
                    );
                    write!(
                        svg,
                        "<g><title>{} / {}: {}</title>",
                        escape(&point.id),
                        escape(&other.id),
                        escape(&label)
                    )
                    .unwrap();
                    let size = (cell - 4.) / font.width(&label, 10.).max(1.) * 10.;
                    svg.push_str(&font.text(
                        &label,
                        x + cell / 2.,
                        row + cell * 0.65,
                        size.min(11.),
                        color,
                        "middle",
                    ));
                    svg.push_str("</g>");
                }
            }
        }
        svg.push_str("</g>");
        *y += ordered.len() as f64 * cell + 15.;
    }
    pub fn aspects(
        &self,
        svg: &mut String,
        scene: &scene::Scene,
        aspects: &[Value],
        rows: usize,
        y: &mut f64,
    ) {
        let (font, p, options) = (self.font, self.paint, self.options);
        let vi = options.locale.as_deref().unwrap_or("vi") == "vi";
        heading(
            svg,
            &format!(
                "{} · {} / {}",
                if vi {
                    "Danh sách góc chiếu"
                } else {
                    "Aspect list"
                },
                rows,
                aspects.len()
            ),
            font,
            p,
            *y,
        );
        for (x, text) in [
            (
                80.,
                if vi {
                    "Điểm 1 / Điểm 2"
                } else {
                    "Point 1 / Point 2"
                },
            ),
            (460., if vi { "Loại" } else { "Type" }),
            (660., "Orb"),
            (760., "Max orb"),
            (870., if vi { "Trạng thái" } else { "Phase" }),
        ] {
            svg.push_str(&font.text(text, x, *y + 62., 11., p.muted, "start"));
        }
        *y += 90.;
        svg.push_str("<g data-section=\"aspect-table\">");
        for (i, a) in aspects.iter().take(rows).enumerate() {
            if i % 2 == 0 {
                write!(
                    svg,
                    "<rect x=\"64\" y=\"{}\" width=\"872\" height=\"29\" rx=\"4\" fill=\"{}\"/>",
                    *y - 19.,
                    p.panel
                )
                .unwrap();
            }
            let first = scene::endpoint(a, true, &scene.key).unwrap();
            let second = scene::endpoint(a, false, &scene.key).unwrap();
            let label = format!("{first} / {second}");
            let size = (350. / font.width(&label, 12.).max(1.)).min(1.) * 12.;
            svg.push_str(&font.text(&label, 80., *y, size, p.text, "start"));
            let angle = a["angle"].as_f64().unwrap();
            let color = styles::color(
                angle,
                options.theme.as_deref() == Some("light"),
                &options.aspect_colors,
            );
            let label = format!("{} {angle}°", styles::name(angle, vi));
            let size = (180. / font.width(&label, 12.).max(1.)).min(1.) * 12.;
            svg.push_str(&font.text(&label, 460., *y, size, color, "start"));
            for (x, key) in [(660., "orb"), (760., "maxOrb")] {
                let value = a[key]
                    .as_f64()
                    .filter(|n| n.is_finite())
                    .map(|n| format!("{n:.4}°"))
                    .unwrap_or_else(|| "—".into());
                svg.push_str(&font.text(&value, x, *y, 11., p.text, "start"));
            }
            let phase = match (a["applying"].as_bool(), vi) {
                (Some(true), true) => "Tiến tới",
                (Some(false), true) => "Tách ra",
                (Some(true), false) => "Applying",
                (Some(false), false) => "Separating",
                _ => "—",
            };
            svg.push_str(&font.text(phase, 870., *y, 10., p.muted, "start"));
            *y += 29.;
        }
        svg.push_str("</g>");
        *y += 30.;
    }
}

impl Panels<'_> {
    pub fn positions(
        &self,
        svg: &mut String,
        scene: &scene::Scene,
        selected: &std::collections::BTreeSet<String>,
        section_y: &mut f64,
    ) {
        let (font, p, options) = (self.font, self.paint, self.options);
        let locale = options.locale.as_deref().unwrap_or("vi");
        let visible = |point: &scene::Point| crate::model::visible(point, options, selected);
        let sign_names = if locale == "vi" {
            [
                "Bạch Dương",
                "Kim Ngưu",
                "Song Tử",
                "Cự Giải",
                "Sư Tử",
                "Xử Nữ",
                "Thiên Bình",
                "Bọ Cạp",
                "Nhân Mã",
                "Ma Kết",
                "Bảo Bình",
                "Song Ngư",
            ]
        } else {
            [
                "Aries",
                "Taurus",
                "Gemini",
                "Cancer",
                "Leo",
                "Virgo",
                "Libra",
                "Scorpio",
                "Sagittarius",
                "Capricorn",
                "Aquarius",
                "Pisces",
            ]
        };
        for (x, label) in [
            (
                105.,
                if locale == "vi" {
                    "Thiên thể"
                } else {
                    "Body"
                },
            ),
            (415., if locale == "vi" { "Cung" } else { "Sign" }),
            (
                640.,
                if locale == "vi" {
                    "Tọa độ"
                } else {
                    "Position"
                },
            ),
            (844., if locale == "vi" { "Nhà" } else { "House" }),
        ] {
            svg.push_str(&font.text(label, x, *section_y + 20., 12., p.muted, "start"));
        }
        line(
            svg,
            (64., *section_y + 32.),
            (936., *section_y + 32.),
            p.line,
            1.,
            "",
        );
        let mut y = *section_y + 57.;
        let mut row = 0;
        for layer in &scene.layers {
            for point in layer.points.iter().filter(|p| visible(p)) {
                if row % 2 == 0 {
                    write!(svg, "<rect x=\"64\" y=\"{}\" width=\"872\" height=\"35\" rx=\"5\" fill=\"{}\"/>", y - 23., p.panel).unwrap();
                }
                row += 1;
                svg.push_str(&symbols::draw(
                    &point.body,
                    77.,
                    y - 5.,
                    22.,
                    body_color(&point.body, false, p),
                    &options.planet_symbols,
                ));
                let name = if scene.layers.len() > 1 {
                    format!(
                        "{} · {}",
                        layer.key,
                        styles::point_name(&point.body, locale == "vi")
                    )
                } else {
                    styles::point_name(&point.body, locale == "vi").into()
                };
                svg.push_str(&font.text(&name, 105., y, 16., p.text, "start"));
                let sign = (point.longitude / 30.).floor() as usize;
                svg.push_str(&symbols::draw(
                    SIGNS[sign],
                    390.,
                    y - 5.,
                    19.,
                    p.violet,
                    &options.zodiac_symbols,
                ));
                svg.push_str(&font.text(sign_names[sign], 415., y, 14., p.text, "start"));
                svg.push_str(&font.text(&dms(point.longitude), 640., y, 15., p.text, "start"));
                let house = point
                    .house
                    .map(|h| format!("H{h}"))
                    .unwrap_or_else(|| "—".into());
                svg.push_str(&font.text(&house, 870., y, 13., p.muted, "end"));
                if point.retrograde == Some(true) {
                    svg.push_str(&font.text("Rx", 930., y, 12., p.peach, "end"));
                }
                y += 35.;
            }
        }
        *section_y += 88. + row as f64 * 35.;
    }
}

fn fit(font: &Font, text: &str, size: f64, width: f64) -> f64 {
    size * (width / font.width(text, size).max(1.)).min(1.)
}
fn summary(ids: &Value) -> String {
    let ids = ids.as_array().unwrap();
    let mut out = ids
        .iter()
        .take(4)
        .filter_map(Value::as_str)
        .collect::<Vec<_>>()
        .join(" · ");
    if ids.len() > 4 {
        out += &format!(" · +{}", ids.len() - 4);
    }
    if out.is_empty() {
        "—".into()
    } else {
        out
    }
}
impl Panels<'_> {
    pub fn house_details(
        &self,
        svg: &mut String,
        scene: &scene::Scene,
        houses: &[Value],
        y: &mut f64,
    ) {
        let (font, p) = (self.font, self.paint);
        let vi = self.options.locale.as_deref().unwrap_or("vi") == "vi";
        heading(
            svg,
            if vi {
                "Nhà · chủ tinh · các điểm bên trong"
            } else {
                "Houses · rulers · occupants"
            },
            font,
            p,
            *y,
        );
        *y += 70.;
        for h in houses {
            let id = h["id"].as_str().unwrap();
            write!(svg,"<g data-house-id=\"{}\"><rect x=\"64\" y=\"{}\" width=\"872\" height=\"78\" rx=\"8\" fill=\"{}\"/>",escape(id),*y-22.,p.panel).unwrap();
            svg.push_str(&font.text(id, 80., *y, 16., p.text, "start"));
            svg.push_str(&font.text(
                &format!(
                    "{} · {}",
                    crate::SIGNS[(h["longitude"].as_f64().unwrap() / 30.).floor() as usize],
                    dms(h["longitude"].as_f64().unwrap())
                ),
                220.,
                *y,
                13.,
                p.muted,
                "start",
            ));
            let ruler = h["rulerId"].as_str().unwrap_or("—");
            let label = format!("{}: {ruler}", if vi { "Chủ tinh" } else { "Ruler" });
            svg.push_str(&font.text(
                &label,
                620.,
                *y,
                fit(font, &label, 13., 290.),
                p.violet,
                "start",
            ));
            let label = format!(
                "{} ({}): {}",
                if vi { "Các điểm" } else { "Points" },
                h["occupantIds"].as_array().unwrap().len(),
                summary(&h["occupantIds"])
            );
            svg.push_str(&font.text(
                &label,
                80.,
                *y + 29.,
                fit(font, &label, 12., 830.),
                p.text,
                "start",
            ));
            write!(svg, "<title>{}</title></g>", escape(&h.to_string())).unwrap();
            *y += 86.;
        }
        svg.push_str(&font.text(
            if vi {
                "Các trục · tọa độ gốc"
            } else {
                "Angles · source coordinates"
            },
            80.,
            *y + 10.,
            15.,
            p.text,
            "start",
        ));
        *y += 40.;
        for layer in &scene.layers {
            for (id, n) in &layer.angles {
                svg.push_str(&font.text(
                    &format!("{} · {id}", layer.key),
                    80.,
                    *y,
                    13.,
                    p.text,
                    "start",
                ));
                svg.push_str(&font.text(
                    &format!("{} · {}", crate::SIGNS[(n / 30.).floor() as usize], dms(*n)),
                    430.,
                    *y,
                    13.,
                    p.muted,
                    "start",
                ));
                *y += 28.;
            }
        }
        *y += 20.;
    }
    pub fn house_focus(&self, svg: &mut String, focus: &Value, aspect_count: usize, y: &mut f64) {
        let (font, p) = (self.font, self.paint);
        let vi = self.options.locale.as_deref().unwrap_or("vi") == "vi";
        let h = &focus["house"];
        heading(
            svg,
            &format!(
                "{} · {}",
                if vi { "Khám phá nhà" } else { "House focus" },
                h["id"].as_str().unwrap()
            ),
            font,
            p,
            *y,
        );
        *y += 70.;
        let n = h["longitude"].as_f64().unwrap();
        svg.push_str(&font.text(
            &format!("{} · {}", crate::SIGNS[(n / 30.).floor() as usize], dms(n)),
            80.,
            *y,
            18.,
            p.text,
            "start",
        ));
        let ruler = h["rulerId"].as_str().unwrap();
        let ruler_house = &h["rulerPlacement"]["house"];
        svg.push_str(&font.text(
            &format!(
                "{}: {ruler} · H{} · {}",
                if vi { "Chủ tinh" } else { "Ruler" },
                ruler_house,
                h["rulership"].as_str().unwrap_or("source-defined")
            ),
            80.,
            *y + 32.,
            14.,
            p.violet,
            "start",
        ));
        svg.push_str(&font.text(
            if vi {
                "Các điểm trong nhà:"
            } else {
                "Points inside house:"
            },
            80.,
            *y + 65.,
            13.,
            p.muted,
            "start",
        ));
        let ids = h["occupantIds"].as_array().unwrap();
        let mut row = String::new();
        let mut at = *y + 90.;
        for id in ids.iter().filter_map(Value::as_str) {
            let next = if row.is_empty() {
                id.into()
            } else {
                format!("{row} · {id}")
            };
            if font.width(&next, 13.) > 820. {
                svg.push_str(&font.text(&row, 80., at, 13., p.text, "start"));
                at += 23.;
                row = id.into();
            } else {
                row = next;
            }
        }
        if row.is_empty() {
            row = "—".into();
        }
        svg.push_str(&font.text(&row, 80., at, 13., p.text, "start"));
        svg.push_str(&font.text(
            &format!(
                "{aspect_count} {}",
                if vi {
                    "góc chiếu sau bộ lọc · không thay tọa độ nguồn"
                } else {
                    "filtered aspects · source coordinates unchanged"
                }
            ),
            80.,
            *y + 190.,
            12.,
            p.muted,
            "start",
        ));
        *y += 250.;
    }
}
