//! HH:MM (assumed on `reference_date`) or full RFC3339 timestamp parsing for
//! `--from` / `--to` / `--at`. Natural-language parsing is out of scope.

use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};

pub fn parse_time(s: &str, reference_date: NaiveDate) -> anyhow::Result<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    let time = NaiveTime::parse_from_str(s, "%H:%M").map_err(|_| {
        anyhow::anyhow!("could not parse \"{s}\" as HH:MM or a full RFC3339 timestamp")
    })?;

    let local = chrono::Local
        .from_local_datetime(&reference_date.and_time(time))
        .single()
        .ok_or_else(|| {
            anyhow::anyhow!("\"{s}\" on {reference_date} is ambiguous in the local timezone")
        })?;

    Ok(local.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, 7).unwrap()
    }

    #[test]
    fn parses_hh_mm_on_reference_date_as_local_then_converts_to_utc() {
        let parsed = parse_time("09:30", a_date()).unwrap();
        // Local 09:30 on 2026-03-07, converted to UTC — assert via round trip
        // through the local offset rather than a hardcoded UTC hour (avoids
        // baking in the test machine's timezone).
        let expected_local = chrono::Local
            .from_local_datetime(&a_date().and_hms_opt(9, 30, 0).unwrap())
            .unwrap();
        assert_eq!(parsed, expected_local.with_timezone(&Utc));
    }

    #[test]
    fn parses_full_rfc3339_timestamp() {
        let parsed = parse_time("2026-03-07T09:30:00Z", a_date()).unwrap();
        assert_eq!(parsed, Utc.with_ymd_and_hms(2026, 3, 7, 9, 30, 0).unwrap());
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_time("not a time", a_date()).is_err());
    }

    #[test]
    fn rejects_invalid_hh_mm() {
        assert!(parse_time("25:99", a_date()).is_err());
    }
}
