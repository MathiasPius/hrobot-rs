use std::{collections::HashMap, net::IpAddr};

use bytesize::ByteSize;
use ipnet::IpNet;
use serde::Deserialize;
use time::{Date, Month};

use crate::{error::Error, urlencode::UrlEncode, AsyncRobot};

use super::{wrapper::Single, UnauthenticatedRequest};

fn get_traffic(
    range: TimeRange,
    ips: &[IpNet],
) -> UnauthenticatedRequest<Single<StatisticContainer>> {
    #[derive(Debug, Clone)]
    struct TrafficRequest<'a> {
        range: TimeRange,
        ips: &'a [IpNet],
    }

    impl<'a> UrlEncode for TrafficRequest<'a> {
        fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
            self.range.encode_into(f.clone());

            let mut addresses = self.ips.to_vec();
            // Sort the list according to whether the address is a single IP address
            // or a subnet. That way we can encode as "ip[]" or "subnet[]" and have
            // each category grouped, instead of interspersing them.
            addresses.sort_by_key(|a| a.max_prefix_len() == a.prefix_len());

            for ip in addresses {
                if ip.prefix_len() == ip.max_prefix_len() {
                    f.set("ip[]", ip.addr());
                } else {
                    f.set("subnet[]", ip);
                }
            }
            f.set("single_values", "true");
        }
    }

    UnauthenticatedRequest::from("https://robot-ws.your-server.de/traffic")
        .with_method("POST")
        .with_serialized_body(TrafficRequest { ips, range }.encode())
}

impl AsyncRobot {
    /// Get traffic statistics for specific IPs
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::traffic::TimeRange;
    /// # use hrobot::time::Month;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let traffic = robot.get_traffic(
    ///     &[
    ///         "123.123.123.123/32".parse().unwrap(),
    ///         "2a01:4f8:123:123::".parse().unwrap(),
    ///     ],
    ///     TimeRange::month(2023, Month::July)
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_traffic(
        &self,
        ips: &[IpNet],
        range: TimeRange,
    ) -> Result<HashMap<IpNet, Vec<TrafficStatistic>>, Error> {
        let result = self.go(get_traffic(range, ips)).await?.0;

        Ok(result
            .data
            .into_iter()
            .map(|(addr, results)| {
                let mut results: Vec<_> = results.into_iter().collect();

                let addr = addr.parse().unwrap_or_else(|_| {
                    let addr: IpAddr = addr.parse().unwrap();
                    IpNet::from(addr)
                });

                results.sort_by(|(a, _), (b, _)| a.cmp(b));
                (
                    addr,
                    results
                        .into_iter()
                        .map(|(_, statistic)| statistic)
                        .collect(),
                )
            })
            .collect())
    }
}

/// Traffic statistics for a single "unit". For hourly range, this is a single hour. For monthly it's a day, for yearly it's a month.
#[derive(Debug, Clone, Deserialize)]
pub struct TrafficStatistic {
    #[serde(rename = "in", deserialize_with = "crate::bytes::gib_float")]
    pub ingress: ByteSize,
    #[serde(rename = "out", deserialize_with = "crate::bytes::gib_float")]
    pub egress: ByteSize,
    #[serde(rename = "sum", deserialize_with = "crate::bytes::gib_float")]
    pub total: ByteSize,
}

#[derive(Debug, Clone, Deserialize)]
struct StatisticContainer {
    data: HashMap<String, HashMap<String, TrafficStatistic>>,
}

/// Describes a time range used to retrieve traffic statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeRange {
    /// Range within a single day.
    Hourly {
        /// The date within which to retrieve aggregated hourly statistics for.
        date: Date,
        /// Start hour.
        from: u8,
        /// End hour.
        to: u8,
    },
    /// Range within a single month
    Daily {
        /// Year within which to retrieve traffic statistics.
        year: u32,
        /// Month within which to retrieve traffic statistics.
        month: Month,
        /// Start day of month (1-32)
        from: u8,
        /// End day of month (1-32)
        to: u8,
    },
    /// Range within a single year.
    Monthly { year: u32, from: Month, to: Month },
}

impl TimeRange {
    pub fn day(date: Date) -> Self {
        TimeRange::Hourly {
            date,
            from: 0,
            to: 24,
        }
    }

    pub fn month(year: u32, month: Month) -> Self {
        TimeRange::Daily {
            year,
            month,
            from: 1,
            to: time::util::days_in_year_month(year as i32, month),
        }
    }

    pub fn year(year: u32) -> Self {
        TimeRange::Monthly {
            year,
            from: Month::January,
            to: Month::December,
        }
    }
}

impl UrlEncode for TimeRange {
    fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
        match self {
            TimeRange::Hourly { date, from, to } => {
                f.set("type", "day");
                f.set(
                    "from",
                    format!(
                        "{year}-{month:0>2}-{day:0>2}T{from:0>2}",
                        year = date.year(),
                        month = date.month() as u8,
                        day = date.day()
                    ),
                );

                f.set(
                    "to",
                    format!(
                        "{year}-{month:0>2}-{day:0>2}T{to:0>2}",
                        year = date.year(),
                        month = date.month() as u8,
                        day = date.day()
                    ),
                );
            }
            TimeRange::Daily {
                year,
                month,
                from,
                to,
            } => {
                f.set("type", "month");
                f.set(
                    "from",
                    format!("{year}-{month:0>2}-{from:0>2}", month = *month as u8),
                );
                f.set(
                    "to",
                    format!("{year}-{month:0>2}-{to:0>2}", month = *month as u8),
                );
            }
            TimeRange::Monthly { year, from, to } => {
                f.set("type", "year");
                f.set("from", format!("{year}-{from:0>2}", from = *from as u8));
                f.set("to", format!("{year}-{to:0>2}", to = *to as u8));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StatisticContainer;
    use time::{Date, Month};
    use tracing_test::traced_test;

    use crate::urlencode::UrlEncode;

    use super::TimeRange;

    #[test]
    #[traced_test]
    fn serialize_date_ranges() {
        assert_eq!(
            TimeRange::year(2022).encode(),
            "type=year&from=2022-01&to=2022-12"
        );

        assert_eq!(
            TimeRange::month(2022, Month::July).encode(),
            "type=month&from=2022-07-01&to=2022-07-31"
        );

        assert_eq!(
            TimeRange::day(Date::from_calendar_date(2022, Month::July, 15).unwrap()).encode(),
            "type=day&from=2022-07-15T00&to=2022-07-15T24"
        );
    }

    #[test]
    #[traced_test]
    fn deserialize_traffic() {
        let traffic = r#"
         {
                "type":"month",
                "from":"2023-06-01",
                "to":"2023-06-30",
                "data":{
                    "46.4.77.66":{
                        "01":{
                            "in":0,
                            "out":0,
                            "sum":0
                        },
                        "02":{
                            "in":0,
                            "out":0,
                            "sum":0
                        }
                    }
            }
        }"#;

        let _container: StatisticContainer = serde_json::from_str(traffic).unwrap();
    }

    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use super::TimeRange;
        use crate::AsyncRobot;
        use ipnet::IpNet;
        use std::net::IpAddr;
        use time::Month;
        use tracing::info;
        use tracing_test::traced_test;

        #[tokio::test]
        #[traced_test]
        async fn get_traffic_data() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            let addresses: Vec<IpNet> = robot
                .list_servers()
                .await
                .unwrap()
                .into_iter()
                .map(|server| IpNet::from(IpAddr::from(server.ipv4.unwrap())))
                .collect();

            let traffic = robot
                .get_traffic(&addresses, TimeRange::month(2023, Month::July))
                .await
                .unwrap();

            info!("{traffic:#?}");
        }
    }
}
