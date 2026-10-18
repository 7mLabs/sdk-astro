//! Offline civil-time resolution. No host timezone, network or astronomical calculation.
use jiff::{
    civil::DateTime,
    tz::{AmbiguousOffset, Offset, TimeZone},
    SignedDuration,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct TimeError {
    pub code: &'static str,
    pub message: String,
}
impl TimeError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Civil {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    #[serde(default)]
    second: f64,
}
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Disambiguation {
    #[default]
    Reject,
    Earlier,
    Later,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Request {
    local: Civil,
    time_zone: Option<String>,
    utc_offset_seconds: Option<i32>,
    #[serde(default)]
    disambiguation: Disambiguation,
}

/// Resolve a local Gregorian time using bundled IANA data or an explicit second offset.
/// Gaps are always rejected. A fold requires an explicit earlier/later choice.
pub fn resolve(input: &Value) -> Result<Value, TimeError> {
    resolve_json(&input.to_string())
}

/// Raw JSON entry point preserves integer validation before any float conversion.
pub fn resolve_json(text: &str) -> Result<Value, TimeError> {
    let req: Request = serde_json::from_str(text)
        .map_err(|e| TimeError::new("INVALID_TIME_INPUT", e.to_string()))?;
    let original: Value = serde_json::from_str(text)
        .map_err(|e| TimeError::new("INVALID_TIME_INPUT", e.to_string()))?;
    let c = &req.local;
    if !(1800..=2399).contains(&c.year) || !c.second.is_finite() || !(0.0..60.0).contains(&c.second)
    {
        return Err(TimeError::new("INVALID_TIME_INPUT", "Local year must be 1800..2399; second must be finite in [0,60). Leap-second input is unsupported."));
    }
    if !(1..=12).contains(&c.month) || !(1..=31).contains(&c.day) || c.hour > 23 || c.minute > 59 {
        return Err(TimeError::new(
            "INVALID_TIME_INPUT",
            "Invalid local date/time components",
        ));
    }
    let whole = c.second.floor() as i8;
    // Preserve representable input fractions to nanosecond resolution, never round to minutes.
    let nanos = ((c.second - whole as f64) * 1_000_000_000.).round() as i64;
    let local = DateTime::new(
        c.year as i16,
        c.month as i8,
        c.day as i8,
        c.hour as i8,
        c.minute as i8,
        whole,
        0,
    )
    .and_then(|d| d.checked_add(SignedDuration::from_nanos(nanos)))
    .map_err(|e| TimeError::new("INVALID_TIME_INPUT", e.to_string()))?;
    let (timestamp, offset, zone, resolution, db_version) = match (
        &req.time_zone,
        req.utc_offset_seconds,
    ) {
        (Some(name), None) => {
            // Read only the pinned bundled database, never TZ, OS zoneinfo or network.
            let (canonical, bytes) = jiff_tzdb::get(name)
                .ok_or_else(|| TimeError::new("UNKNOWN_TIME_ZONE", "Unknown IANA timeZone"))?;
            let tz = TimeZone::tzif(canonical, bytes)
                .map_err(|e| TimeError::new("TIME_DATA_ERROR", e.to_string()))?;
            let ambiguous = tz.to_ambiguous_timestamp(local);
            let (chosen, resolution) = match ambiguous.offset() {
                AmbiguousOffset::Unambiguous {..} => (ambiguous.unambiguous(), "unique"),
                AmbiguousOffset::Fold {..} => match req.disambiguation {
                    Disambiguation::Earlier => (ambiguous.earlier(), "earlier"),
                    Disambiguation::Later => (ambiguous.later(), "later"),
                    Disambiguation::Reject => return Err(TimeError::new("AMBIGUOUS_LOCAL_TIME", "Local time occurs twice. Select disambiguation earlier/later or supply a verified utcOffsetSeconds.")),
                },
                AmbiguousOffset::Gap {..} => return Err(TimeError::new("NONEXISTENT_LOCAL_TIME", "Local time does not exist in this zone. Correct the time or supply a verified explicit offset; the SDK does not shift it silently.")),
            };
            let timestamp =
                chosen.map_err(|e| TimeError::new("TIME_OUT_OF_RANGE", e.to_string()))?;
            (
                timestamp,
                tz.to_offset(timestamp).seconds(),
                Some(canonical),
                resolution,
                jiff_tzdb::VERSION,
            )
        }
        (None, Some(offset)) if (-86_399..=86_399).contains(&offset) => {
            if !matches!(req.disambiguation, Disambiguation::Reject) {
                return Err(TimeError::new(
                    "INVALID_TIME_INPUT",
                    "disambiguation only applies to an IANA timeZone",
                ));
            }
            (
                Offset::from_seconds(offset)
                    .and_then(|o| o.to_timestamp(local))
                    .map_err(|e| TimeError::new("TIME_OUT_OF_RANGE", e.to_string()))?,
                offset,
                None,
                "explicitOffset",
                None,
            )
        }
        _ => return Err(TimeError::new(
            "INVALID_TIME_INPUT",
            "Provide exactly one of timeZone or utcOffsetSeconds (integer, absolute value <86400).",
        )),
    };
    let utc = Offset::UTC.to_datetime(timestamp);
    if !(1800..=2399).contains(&utc.year()) {
        return Err(TimeError::new(
            "TIME_OUT_OF_RANGE",
            "Resolved UTC is outside engine coverage 1800..2399",
        ));
    }
    let mut warnings = if c.year < 1970 {
        vec!["Historical timezone records, especially before 1970, can be incomplete. Verify the recorded local time and offset when precision matters."]
    } else {
        vec![]
    };
    let data_year = db_version
        .and_then(|v| v.get(..4))
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    if c.year > data_year && zone.is_some() {
        warnings.push("Future civil offsets follow the bundled tzdb's current rules, not a prediction of future political timezone changes.");
    }
    Ok(json!({
        "utc": {"year":utc.year(),"month":utc.month(),"day":utc.day(),"hour":utc.hour(),"minute":utc.minute(),"second":utc.second() as f64 + utc.subsec_nanosecond() as f64 / 1_000_000_000.},
        "local": original["local"], "timeZone": zone, "utcOffsetSeconds": offset,
        "tzdbVersion": db_version, "resolution": resolution,
        "precision": "nanosecond civil conversion; astronomical input remains f64 seconds",
        "warnings": warnings
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(y: i32, m: u32, d: u32, h: u32, min: u32, zone: &str) -> Value {
        json!({"local":{"year":y,"month":m,"day":d,"hour":h,"minute":min},"timeZone":zone})
    }
    #[test]
    fn vietnam_rollover_and_fractional_seconds() {
        let mut r = request(1995, 6, 15, 2, 30, "Asia/Ho_Chi_Minh");
        r["local"]["second"] = json!(12.125);
        let x = resolve(&r).unwrap();
        assert_eq!(
            x["utc"],
            json!({"year":1995,"month":6,"day":14,"hour":19,"minute":30,"second":12.125})
        );
        assert_eq!(x["utcOffsetSeconds"], 25200);
        assert!(x["tzdbVersion"].is_string());
    }
    #[test]
    fn fold_requires_choice_and_gap_is_never_shifted() {
        let mut r = request(2024, 11, 3, 1, 30, "America/New_York");
        assert_eq!(resolve(&r).unwrap_err().code, "AMBIGUOUS_LOCAL_TIME");
        r["disambiguation"] = json!("earlier");
        let a = resolve(&r).unwrap();
        r["disambiguation"] = json!("later");
        let b = resolve(&r).unwrap();
        assert_eq!(a["utc"]["hour"], 5);
        assert_eq!(b["utc"]["hour"], 6);
        let mut gap = request(2024, 3, 10, 2, 30, "America/New_York");
        gap["disambiguation"] = json!("later");
        assert_eq!(resolve(&gap).unwrap_err().code, "NONEXISTENT_LOCAL_TIME");
    }
    #[test]
    fn non_hour_dst_and_skipped_calendar_day() {
        let r = request(2024, 10, 6, 2, 15, "Australia/Lord_Howe");
        assert_eq!(resolve(&r).unwrap_err().code, "NONEXISTENT_LOCAL_TIME");
        assert_eq!(
            resolve(&request(2011, 12, 30, 12, 0, "Pacific/Apia"))
                .unwrap_err()
                .code,
            "NONEXISTENT_LOCAL_TIME"
        );
    }
    #[test]
    fn explicit_seconds_offset_and_invalid_inputs() {
        let mut r = request(1900, 1, 1, 0, 0, "Etc/UTC");
        r.as_object_mut().unwrap().remove("timeZone");
        r["utcOffsetSeconds"] = json!(2079);
        let x = resolve(&r).unwrap();
        assert_eq!(x["utc"]["year"], 1899);
        assert_eq!(x["utc"]["second"], 21.0);
        r["timeZone"] = json!("Etc/UTC");
        assert!(resolve(&r).is_err());
        assert!(resolve(&request(2023, 2, 29, 12, 0, "Etc/UTC")).is_err());
        assert_eq!(
            resolve(&request(2024, 1, 1, 12, 0, "Wrong/Zone"))
                .unwrap_err()
                .code,
            "UNKNOWN_TIME_ZONE"
        );
        assert!(resolve(&request(1800, 1, 1, 0, 0, "Asia/Ho_Chi_Minh")).is_err());
    }
    #[test]
    fn recurring_dst_after_2099_is_not_frozen_to_last_offset() {
        for year in [2099, 2100, 2200, 2399] {
            let winter = resolve(&request(year, 1, 15, 12, 0, "America/New_York")).unwrap();
            let summer = resolve(&request(year, 7, 15, 12, 0, "America/New_York")).unwrap();
            assert_eq!(winter["utcOffsetSeconds"], -18000);
            assert_eq!(summer["utcOffsetSeconds"], -14400);
        }
    }
}
