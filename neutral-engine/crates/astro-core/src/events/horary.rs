//! Lunar perfections strictly between the question and the next tropical ingress.
use super::*;

const MOON: usize = 1;
const OTHER_CLASSICAL_BODIES: [usize; 6] = [0, 2, 3, 4, 5, 6];
const LUNAR_ROOT_TOLERANCE: f64 = 1e-8; // degrees; finer than the 1 ms ordering policy

/// In apparent geocentric longitude the Moon overtakes all six selected bodies.
/// During one sign passage every oriented aspect branch can be crossed at most
/// once. The four-day ingress bracket spans less than 90 degrees of lunar travel.
fn next_crossing(
    sampler: &mut Sampler,
    model: Model,
    start: f64,
    end: f64,
    branch: f64,
) -> Result<Option<f64>, String> {
    let (origin, start_speed) = model.evaluate(sampler, start)?;
    let (finish, end_speed) = model.evaluate(sampler, end)?;
    let travel = signed_difference(finish, origin);
    if start_speed <= 0. || end_speed <= 0. || !(0.0..90.0).contains(&travel) {
        return Err("Lunar search exceeded its forward-motion bracket".into());
    }
    let distance = normalize(branch - origin);
    if distance == 0. || distance > travel {
        return Ok(None);
    }
    let root = refine(
        start,
        end,
        -distance,
        travel - distance,
        LUNAR_ROOT_TOLERANCE,
        |jd| Ok(signed_difference(model.evaluate(sampler, jd)?.0, origin) - distance),
    )?;
    Ok(Some(root.t))
}

pub(crate) fn horary_moon(start: f64, rules: &[AspectRule]) -> Result<Value, String> {
    let mut sampler = Sampler {
        session: EphemerisSession::new()?,
        cache: HashMap::new(),
        evaluations: 0,
        window: None,
    };
    let moon = Model {
        first: MOON,
        second: None,
        fixed: 0.,
    };
    let longitude = moon.evaluate(&mut sampler, start)?.0;
    let sign = (longitude / 30.).floor() as usize;
    let next_sign = (sign + 1) % 12;
    let ingress = next_crossing(
        &mut sampler,
        moon,
        start,
        start + 4.,
        30. * next_sign as f64,
    )?
    .ok_or("The Moon did not enter the next sign within four days")?;
    let mut earliest: Option<(f64, usize, f64)> = None;
    let branches = aspect_branches(rules);
    for other in OTHER_CLASSICAL_BODIES {
        let relative = Model {
            first: MOON,
            second: Some(other),
            fixed: 0.,
        };
        for &(branch, angle) in &branches {
            if let Some(jd) = next_crossing(&mut sampler, relative, start, ingress, branch)? {
                // Same one-millisecond identity threshold as the event scanner.
                if (jd - start) * 86400. > DUPLICATE_SECONDS
                    && (ingress - jd) * 86400. > DUPLICATE_SECONDS
                    && earliest.is_none_or(|(previous, _, _)| jd < previous)
                {
                    earliest = Some((jd, other, angle));
                }
            }
        }
    }
    let aspect = if let Some((jd, other, angle)) = earliest {
        Some(
            json!({"utc":sampler.times(jd, 0)?.0,"julianDayUt1":jd,"otherBodyId":BODY_IDS[other],"angle":angle}),
        )
    } else {
        None
    };
    Ok(json!({
        "method":"exactMajorAspectBeforeNextSignIngress",
        "currentSign":SIGNS[sign],
        "isVoidOfCourse":aspect.is_none(),
        "nextAspect":aspect,
        "nextIngress":{"utc":sampler.times(ingress, 0)?.0,"julianDayUt1":ingress,"toSign":SIGNS[next_sign]}
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jd(utc: [i32; 5], second: f64) -> f64 {
        EphemerisSession::new()
            .unwrap()
            .julian_days(utc, second)
            .unwrap()[1]
    }

    #[test]
    fn full_moon_matches_independent_usno_minute() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../tests/fixtures/usno-moon-phases-2026.json"
        ))
        .unwrap();
        let reference = &fixture["phases"][0];
        assert_eq!(reference["phase"], "fullMoon");
        let expected = jd(
            [
                fixture["year"].as_i64().unwrap() as i32,
                reference["month"].as_i64().unwrap() as i32,
                reference["day"].as_i64().unwrap() as i32,
                reference["hour"].as_i64().unwrap() as i32,
                reference["minute"].as_i64().unwrap() as i32,
            ],
            0.,
        );
        let moon = horary_moon(expected - 300. / 86400., &default_rules()).unwrap();
        let aspect = &moon["nextAspect"];
        assert_eq!(aspect["otherBodyId"], "sun");
        assert_eq!(aspect["angle"], 180.);
        assert!(
            (aspect["julianDayUt1"].as_f64().unwrap() - expected).abs() * 86400.
                <= fixture["toleranceSeconds"].as_f64().unwrap()
        );
    }

    #[test]
    fn last_contact_and_ingress_switches_agree_with_released_event_scanner() {
        // Times from the 1.1.3 general event scanner, using its separate 6h scan.
        let rules = default_rules();
        let saturn_square = jd([2026, 1, 2, 7, 9], 27.828);
        let ingress = jd([2026, 1, 2, 13, 8], 59.866);
        let second = 1. / 86400.;
        let before = horary_moon(saturn_square - second, &rules).unwrap();
        let aspect = &before["nextAspect"];
        assert_eq!(aspect["otherBodyId"], "saturn");
        assert_eq!(aspect["angle"], 90.);
        let exact = aspect["julianDayUt1"].as_f64().unwrap();
        assert!((exact - saturn_square).abs() < 0.25 * second);
        assert_eq!(before["isVoidOfCourse"], false);
        // A contact just reached is not a future perfection. UTC evidence is
        // millisecond precision; events tied at that precision are excluded.
        for t in [
            exact - 0.0005 * second,
            exact,
            exact + second,
            ingress - second,
        ] {
            let moon = horary_moon(t, &rules).unwrap();
            assert_eq!(moon["isVoidOfCourse"], true);
            assert_eq!(moon["nextAspect"], Value::Null);
            assert_eq!(moon["currentSign"], "Gemini");
            assert_eq!(moon["nextIngress"]["toSign"], "Cancer");
            assert!(
                (moon["nextIngress"]["julianDayUt1"].as_f64().unwrap() - ingress).abs()
                    < 0.25 * second
            );
        }
        let after = horary_moon(ingress + second, &rules).unwrap();
        assert_eq!(after["isVoidOfCourse"], false);
        assert_eq!(after["currentSign"], "Cancer");
        assert_eq!(after["nextAspect"]["otherBodyId"], "mercury");
        assert_eq!(after["nextAspect"]["angle"], 180.);
    }

    #[test]
    fn valid_edge_years_include_lunar_evidence_until_the_next_ingress() {
        let rules = default_rules();
        for epoch in [[1800, 1, 1, 0, 0], [2399, 12, 31, 23, 59]] {
            let start = jd(epoch, 0.);
            let moon = horary_moon(start, &rules).unwrap();
            let ingress = moon["nextIngress"]["julianDayUt1"].as_f64().unwrap();
            assert!(ingress > start && ingress < start + 4.);
            if epoch[0] == 2399 {
                assert_eq!(moon["nextIngress"]["utc"]["year"], 2400);
            }
        }
    }
}
