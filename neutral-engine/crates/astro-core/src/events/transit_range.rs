//! Continuous longitude contacts against fixed natal longitudes. Each arc is
//! monotone, unwrapped, and shorter than a quadrant, including across 0 Aries.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
struct Knot {
    t: f64,
    x: f64,
}

fn track(s: &mut Sampler, body: usize, start: f64, end: f64) -> Result<Vec<Knot>, String> {
    let model = Model {
        first: body,
        second: None,
        fixed: 0.,
    };
    let mut prev = Knot {
        t: start,
        x: s.position(start, body)?[0],
    };
    let mut knots = vec![prev];
    while prev.t < end {
        let right = (prev.t + SAMPLE_DAYS).min(end);
        let turns = extrema(s, model, prev.t, right)?;
        for (t, turn) in turns
            .iter()
            .map(|r| (r.t, true))
            .chain(std::iter::once((right, false)))
        {
            if t < prev.t {
                continue;
            }
            let lon = s.position(t, body)?[0];
            prev = Knot {
                t,
                x: prev.x + signed_difference(lon, normalize(prev.x)),
            };
            let last = knots.last().unwrap();
            if t > last.t && (turn || t == end || (prev.x - last.x).abs() >= 45.) {
                knots.push(prev);
            }
        }
    }
    Ok(knots)
}

fn at(s: &mut Sampler, body: usize, a: Knot, b: Knot, x: f64) -> Result<f64, String> {
    if (a.x - x).abs() <= TANGENCY_TOLERANCE {
        return Ok(a.t);
    }
    if (b.x - x).abs() <= TANGENCY_TOLERANCE {
        return Ok(b.t);
    }
    Ok(refine(a.t, b.t, a.x - x, b.x - x, ANGULAR_TOLERANCE, |t| {
        Ok(a.x + signed_difference(s.position(t, body)?[0], normalize(a.x)) - x)
    })?
    .t)
}

#[derive(Clone)]
struct Window {
    start: f64,
    end: f64,
    closest: f64,
    orb: f64,
    exact: Vec<f64>,
}
impl Window {
    fn merge(&mut self, next: Self) {
        self.end = self.end.max(next.end);
        if next.orb < self.orb || (next.orb == self.orb && next.closest < self.closest) {
            self.closest = next.closest;
            self.orb = next.orb;
        }
        for t in next.exact {
            if self
                .exact
                .last()
                .is_none_or(|old| (old - t).abs() * 86400. > DUPLICATE_SECONDS)
            {
                self.exact.push(t);
            }
        }
    }
}

fn windows(
    s: &mut Sampler,
    body: usize,
    knots: &[Knot],
    target: f64,
    rule: &AspectRule,
) -> Result<Vec<Window>, String> {
    let low = (rule.angle - rule.max_orb).max(0.);
    let high = (rule.angle + rule.max_orb).min(180.);
    let mut spans = Vec::new();
    for arc in knots.windows(2) {
        let (a, b) = (arc[0], arc[1]);
        let min = a.x.min(b.x);
        let max = a.x.max(b.x);
        for sign in [-1., 1.] {
            if sign < 0. && low == high && (low == 0. || low == 180.) {
                continue;
            }
            let l = target + (sign * low).min(sign * high);
            let h = target + (sign * low).max(sign * high);
            let goal = target + sign * rule.angle;
            let first = ((min - h - TANGENCY_TOLERANCE) / 360.).ceil() as i32;
            let last = ((max - l + TANGENCY_TOLERANCE) / 360.).floor() as i32;
            for cycle in first..=last {
                let shift = 360. * f64::from(cycle);
                let left = (l + shift).clamp(min, max);
                let right = (h + shift).clamp(min, max);
                let (x, y) = if a.x <= b.x {
                    (left, right)
                } else {
                    (right, left)
                };
                let start = at(s, body, a, b, x)?;
                let end = if x == y { start } else { at(s, body, a, b, y)? };
                let exact_lon = goal + shift;
                let closest_lon = exact_lon.clamp(left, right);
                let closest = if closest_lon == x {
                    start
                } else if closest_lon == y {
                    end
                } else {
                    at(s, body, a, b, closest_lon)?
                };
                let orb = (closest_lon - exact_lon).abs();
                let exact = orb <= TANGENCY_TOLERANCE;
                spans.push(Window {
                    start,
                    end,
                    closest,
                    orb: if exact { 0. } else { orb },
                    exact: if exact { vec![closest] } else { vec![] },
                });
            }
        }
    }
    spans.sort_by(|a, b| a.start.total_cmp(&b.start).then(a.end.total_cmp(&b.end)));
    let mut result: Vec<Window> = Vec::new();
    for span in spans {
        if let Some(last) = result.last_mut().filter(|w| span.start <= w.end) {
            last.merge(span)
        } else {
            result.push(span)
        }
    }
    Ok(result)
}

fn stamp(s: &mut Sampler, t: f64) -> Result<Value, String> {
    Ok(json!({"julianDayUt1":t,"utc":s.times(t,0)?.0}))
}

fn boundaries(
    s: &mut Sampler,
    body: usize,
    knots: &[Knot],
    target: &Target,
    house: usize,
    start: f64,
    end: f64,
) -> Result<Vec<Value>, String> {
    let rule = AspectRule {
        angle: 0.,
        max_orb: 0.,
    };
    let mut result = vec![];
    for w in windows(s, body, knots, target.longitude, &rule)? {
        let t = w.closest;
        if t < start || t >= end {
            continue;
        }
        // Use the motion on both adjacent monotone arcs. A reversal on the
        // cusp has opposite incoming/outgoing velocities and the same house.
        let before = knots.windows(2).find(|p| p[0].t < t && t <= p[1].t);
        let after = knots.windows(2).find(|p| p[0].t <= t && t < p[1].t);
        let velocity = s.position(t, body)?[3];
        let motion = |arc: Option<&[Knot]>| arc.map_or(velocity, |p| p[1].x - p[0].x);
        let lower = (house + 10) % 12 + 1;
        let from = if motion(before) > 0. { lower } else { house };
        let to = if motion(after) > 0. { house } else { lower };
        result.push(
            json!({"time":stamp(s,t)?,"bodyId":BODY_IDS[body],"cuspId":target.id,
            "fromHouse":from,"toHouse":to,"kind":if from==to{"touch"}else{"crossing"},
            "motion":if from==to{"stationary"}else if to==house{"direct"}else{"retrograde"}}),
        );
    }
    Ok(result)
}

struct OutputWindow {
    start: f64,
    end: f64,
    id: String,
    themes: Vec<String>,
    value: Value,
}

fn groups(
    s: &mut Sampler,
    intervals: &[OutputWindow],
    start: f64,
    end: f64,
) -> Result<Vec<Value>, String> {
    let mut result = vec![];
    let mut i = 0;
    while i < intervals.len() {
        let first = i;
        let begin = start.max(intervals[i].start);
        let mut stop = intervals[i].end.min(end);
        let mut evidence: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        while i < intervals.len() && intervals[i].start <= stop {
            let w = &intervals[i];
            stop = stop.max(w.end.min(end));
            for theme in &w.themes {
                evidence.entry(theme).or_default().push(&w.id);
            }
            i += 1;
        }
        result.push(json!({"start":stamp(s,begin)?,"end":stamp(s,stop)?,"windowIds":intervals[first..i].iter().map(|w|&w.id).collect::<Vec<_>>(),"themeEvidence":evidence}));
    }
    Ok(result)
}

pub(crate) fn calculate(
    bounds: &PeriodBounds,
    options: &ScanOptions,
    targets: &[Target],
    themes: &BTreeMap<String, Vec<String>>,
) -> Result<Value, String> {
    let mut s = Sampler {
        session: EphemerisSession::new()?,
        cache: HashMap::new(),
        evaluations: 0,
        window: None,
    };
    let start = s
        .session
        .julian_days(utc_args(&bounds.start), bounds.start.second)?[1];
    let end = s
        .session
        .julian_days(utc_args(&bounds.end), bounds.end.second)?[1];
    let floor = s.session.julian_days([1800, 1, 1, 0, 0], 0.)?[1];
    let ceiling = s.session.julian_days([2400, 1, 1, 0, 0], 0.)?[1];
    let mut intervals = vec![];
    let mut crossings = vec![];
    for body in options.indices() {
        let mut pending: BTreeSet<(usize, usize)> = (0..targets.len())
            .flat_map(|i| (0..options.rules.len()).map(move |j| (i, j)))
            .collect();
        let mut pad = 2.;
        loop {
            let a = (start - pad).max(floor);
            let b = (end + pad).min(ceiling);
            let knots = track(&mut s, body, a, b)?;
            if pad == 2. {
                for target in targets {
                    if let Some(h) = target
                        .id
                        .strip_prefix("N:H")
                        .and_then(|v| v.parse::<usize>().ok())
                    {
                        crossings.extend(boundaries(&mut s, body, &knots, target, h, start, end)?);
                    }
                }
            }
            let mut retry = BTreeSet::new();
            for (i, j) in pending {
                let target = &targets[i];
                let rule = &options.rules[j];
                let found: Vec<_> = windows(&mut s, body, &knots, target.longitude, rule)?
                    .into_iter()
                    .filter(|w| w.end >= start && w.start < end)
                    .collect();
                if found
                    .iter()
                    .any(|w| (w.start == a && a > floor) || (w.end == b && b < ceiling))
                {
                    retry.insert((i, j));
                    continue;
                }
                for w in found {
                    if intervals.len() >= MAX_EVENTS {
                        return Err("Transit range exceeded the window limit".into());
                    }
                    let id = format!(
                        "{}:{}:{}:{:.8}",
                        BODY_IDS[body], target.id, rule.angle, w.start
                    );
                    let exact = w
                        .exact
                        .iter()
                        .map(|t| stamp(&mut s, *t))
                        .collect::<Result<Vec<_>, _>>()?;
                    let entry = if w.start == floor {
                        Value::Null
                    } else {
                        stamp(&mut s, w.start)?
                    };
                    let exit = if w.end == ceiling {
                        Value::Null
                    } else {
                        stamp(&mut s, w.end)?
                    };
                    let labels = themes.get(&target.id).cloned().unwrap_or_default();
                    let value = json!({"id":id,"bodyId":BODY_IDS[body],"targetPointId":target.id,"angle":rule.angle,"maxOrb":rule.max_orb,
                        "entry":entry,"exit":exit,"exact":exact,"closest":stamp(&mut s,w.closest)?,"minimumOrb":w.orb,"domainIds":labels});
                    intervals.push(OutputWindow {
                        start: w.start,
                        end: w.end,
                        id,
                        themes: labels,
                        value,
                    });
                }
            }
            if retry.is_empty() {
                break;
            }
            pending = retry;
            pad *= 4.;
        }
        s.cache.clear();
    }
    intervals.sort_by(|a, b| a.start.total_cmp(&b.start).then(a.id.cmp(&b.id)));
    crossings.sort_by(|a, b| {
        a["time"]["julianDayUt1"]
            .as_f64()
            .unwrap()
            .total_cmp(&b["time"]["julianDayUt1"].as_f64().unwrap())
    });
    let phases = groups(&mut s, &intervals, start, end)?;
    Ok(
        json!({"houseBoundaries":crossings,"windows":intervals.into_iter().map(|w|w.value).collect::<Vec<_>>(),"groups":phases}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_on_cusp_touches_while_a_nearby_cusp_is_crossed_twice() {
        let mut sampler = Sampler {
            session: EphemerisSession::new().unwrap(),
            cache: HashMap::new(),
            evaluations: 0,
            window: None,
        };
        let jd = sampler.session.julian_days([2026, 1, 1, 0, 0], 0.).unwrap()[1];
        let body = 2; // Mercury
        let knots = track(&mut sampler, body, jd, jd + 150.).unwrap();
        let turn = knots
            .windows(3)
            .find(|p| p[0].x < p[1].x && p[1].x > p[2].x)
            .unwrap()[1];
        let target = Target {
            id: "N:H1".into(),
            longitude: normalize(turn.x),
        };
        let hits = boundaries(
            &mut sampler,
            body,
            &knots,
            &target,
            1,
            turn.t - 3.,
            turn.t + 3.,
        )
        .unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0]["kind"], "touch");
        assert_eq!(hits[0]["fromHouse"], 12);
        assert_eq!(hits[0]["toHouse"], 12);
        let nearby = Target {
            id: target.id,
            longitude: normalize(turn.x - 0.01),
        };
        let hits = boundaries(
            &mut sampler,
            body,
            &knots,
            &nearby,
            1,
            turn.t - 3.,
            turn.t + 3.,
        )
        .unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(
            (&hits[0]["fromHouse"], &hits[0]["toHouse"]),
            (&json!(12), &json!(1))
        );
        assert_eq!(hits[1]["motion"], "retrograde");
        assert_eq!(hits[1]["toHouse"], 12);
        let rule = AspectRule {
            angle: 0.,
            max_orb: 1.,
        };
        let near_miss = windows(&mut sampler, body, &knots, normalize(turn.x + 0.25), &rule)
            .unwrap()
            .into_iter()
            .find(|w| w.start < turn.t && turn.t < w.end)
            .unwrap();
        assert!(near_miss.exact.is_empty());
        assert!((near_miss.orb - 0.25).abs() < 1e-9);
        assert_eq!(near_miss.closest, turn.t);
        let double = windows(&mut sampler, body, &knots, nearby.longitude, &rule)
            .unwrap()
            .into_iter()
            .find(|w| w.start < turn.t && turn.t < w.end)
            .unwrap();
        assert_eq!(double.exact.len(), 2);
    }
}
