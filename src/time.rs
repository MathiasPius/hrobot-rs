//! Time conversion and assumptions utilities
//!
//! Some of the Hetzner Robot API endpoints return datetimes
//! with no accompanying timezone information.
//!
//! Some of them return UTC timestamps, and others return
//! timestamps which appear to correlate with German local
//! time (Europe/Berlin).

use serde::{de::Error, Deserialize, Deserializer};
use time::{macros::format_description, OffsetDateTime, PrimitiveDateTime};
use time_tz::PrimitiveDateTimeExt;

/// Deserialize as [`OffsetDateTime`](time::OffsetDateTime)
/// based on the assumption that the timezone is Europe/Berlin.
pub(crate) fn assume_berlin_timezone<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<OffsetDateTime, D::Error> {
    let datetime = <&str>::deserialize(deserializer)?;

    Ok(PrimitiveDateTime::parse(
        datetime,
        &format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    )
    .map_err(D::Error::custom)?
    .assume_timezone(time_tz::timezones::db::europe::BERLIN)
    .unwrap())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use time::{macros::datetime, OffsetDateTime, Month, Date};

    #[test]
    fn deserialize_berlin_timestamp() {
        let container = r#"
            {
                "timestamp": "2023-06-10 21:34:12"
            }"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Container {
            #[serde(deserialize_with = "super::assume_berlin_timezone")]
            timestamp: OffsetDateTime,
        }

        assert_eq!(
            Container {
                timestamp: datetime!(2023-06-10 21:34:12 +02:00),
            },
            serde_json::from_str(container).unwrap()
        )
    }

    #[test]
    fn deserialize_date() {
        let container = r#"
            {
                "date": "2023-06-10"
            }"#;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Container {
            date: Date,
        }

        assert_eq!(
            Container {
                date: Date::from_calendar_date(2023, Month::June, 10).unwrap(),
            },
            serde_json::from_str(container).unwrap()
        )
    }

}
