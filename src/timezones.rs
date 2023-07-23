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

pub(crate) mod weekday_plus_one {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::Weekday;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Weekday>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if let Some(day) = Option::<u8>::deserialize(deserializer)? {
            Ok(Some(Weekday::Sunday.nth_next(day)))
        } else {
            Ok(None)
        }
    }

    pub fn serialize<S>(weekday: &Option<Weekday>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(weekday) = weekday {
            serializer.serialize_some(&((weekday.number_days_from_monday() % 7) + 1))
        } else {
            serializer.serialize_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use time::{macros::datetime, Date, Month, OffsetDateTime, Weekday};

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

    #[test]
    fn serde_weekday_offset() {
        #[derive(Serialize, Deserialize, PartialEq, Eq)]
        struct Container(#[serde(with = "crate::timezones::weekday_plus_one")] pub Option<Weekday>);

        assert_eq!(None, serde_json::from_str::<Container>("null").unwrap().0);

        assert_eq!(
            Some(Weekday::Monday),
            serde_json::from_str::<Container>("1").unwrap().0
        );

        assert_eq!(
            Some(Weekday::Wednesday),
            serde_json::from_str::<Container>("3").unwrap().0
        );

        assert_eq!(
            Some(Weekday::Sunday),
            serde_json::from_str::<Container>("7").unwrap().0
        );

        assert_eq!(
            &serde_json::to_string(&Container(Some(Weekday::Monday))).unwrap(),
            "1"
        );

        assert_eq!(
            &serde_json::to_string(&Container(Some(Weekday::Wednesday))).unwrap(),
            "3"
        );

        assert_eq!(
            &serde_json::to_string(&Container(Some(Weekday::Sunday))).unwrap(),
            "7"
        );

        assert_eq!(
            Weekday::Monday,
            serde_json::from_str(
                &serde_json::to_string(&Container(Some(Weekday::Monday))).unwrap()
            )
            .unwrap()
        );

        assert_eq!(
            Weekday::Wednesday,
            serde_json::from_str(
                &serde_json::to_string(&Container(Some(Weekday::Wednesday))).unwrap()
            )
            .unwrap()
        );

        assert_eq!(
            Weekday::Sunday,
            serde_json::from_str(
                &serde_json::to_string(&Container(Some(Weekday::Sunday))).unwrap()
            )
            .unwrap()
        );
    }
}
