use std::{collections::HashMap, fmt::Display};

use bytesize::ByteSize;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use time::OffsetDateTime;

use crate::api::server::ServerId;

#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    /// Unique identifier for this product type.
    pub id: ProductId,

    /// Human-readable name for this product.
    pub name: String,

    /// Human-readable list of features for this product.
    pub description: Vec<String>,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    #[serde(rename = "traffic", deserialize_with = "crate::conversion::traffic")]
    pub traffic_limit: Option<ByteSize>,

    /// Available distributions for this product.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Available languages for this product.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,

    /// Locations where this product is available.
    #[serde(default, rename = "location")]
    pub locations: Vec<Location>,

    /// Prices for this product in each location
    #[serde(deserialize_with = "location_prices")]
    pub prices: HashMap<Location, LocationPrice>,

    /// Addons which can be purchased for this product.
    pub orderable_addons: Vec<Addon>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurchasedProduct {
    /// Unique identifier for this product type.
    pub id: ProductId,

    /// Human-readable name for this product.
    pub name: String,

    /// Human-readable list of features for this product.
    pub description: Vec<String>,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    #[serde(rename = "traffic", deserialize_with = "crate::conversion::traffic")]
    pub traffic_limit: Option<ByteSize>,

    /// Distribution selected for the purchased product.
    #[serde(rename = "dist")]
    pub distribution: String,

    /// Language selected for the product.
    #[serde(rename = "lang")]
    pub language: String,

    /// Location of the purchased product.
    #[serde(rename = "location")]
    pub location: Option<Location>,
}

fn location_prices<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<Location, LocationPrice>, D::Error> {
    let prices = Vec::<InternalLocationPrice>::deserialize(deserializer)?;

    Ok(prices
        .into_iter()
        .map(
            |InternalLocationPrice {
                 location,
                 monthly,
                 setup,
             }| (location, LocationPrice { monthly, setup }),
        )
        .collect())
}

#[derive(Debug, Clone, Deserialize)]
struct InternalLocationPrice {
    pub location: Location,
    #[serde(rename = "price")]
    pub monthly: Price,
    #[serde(rename = "price_setup")]
    pub setup: Price,
}

/// Price (both setup and monthly) for a single location.
#[derive(Debug, Clone)]
pub struct LocationPrice {
    pub monthly: Price,
    pub setup: Price,
}

/// A single price point, both excluding and including VAT.
#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    /// Price excluding VAT.
    pub net: Decimal,
    /// Price including VAT.
    pub gross: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Addon {
    pub id: String,
    pub name: String,
    pub location: Option<Location>,
    pub min: u32,
    pub max: u32,
    #[serde(deserialize_with = "location_prices")]
    pub prices: HashMap<Location, LocationPrice>,
}

/// Location, e.g. "FSN1".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location(pub String);

impl From<String> for Location {
    fn from(value: String) -> Self {
        Location(value)
    }
}

impl From<&str> for Location {
    fn from(value: &str) -> Self {
        Location(value.to_string())
    }
}

impl From<Location> for String {
    fn from(value: Location) -> Self {
        value.0
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for Location {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Datacenter within a Location, e.g. "FSN1-DC1".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Datacenter(pub String);

impl From<String> for Datacenter {
    fn from(value: String) -> Self {
        Datacenter(value)
    }
}

impl From<&str> for Datacenter {
    fn from(value: &str) -> Self {
        Datacenter(value.to_string())
    }
}

impl From<Datacenter> for String {
    fn from(value: Datacenter) -> Self {
        value.0
    }
}

impl From<Datacenter> for Location {
    fn from(value: Datacenter) -> Location {
        Location(value.0.split_once('-').unwrap().0.to_string())
    }
}

impl Display for Datacenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for Datacenter {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Product ID, e.g. "EX44".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(pub String);

impl From<String> for ProductId {
    fn from(value: String) -> Self {
        ProductId(value)
    }
}

impl From<&str> for ProductId {
    fn from(value: &str) -> Self {
        ProductId(value.to_string())
    }
}

impl From<ProductId> for String {
    fn from(value: ProductId) -> Self {
        value.0
    }
}

impl Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for ProductId {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,

    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,

    pub status: TransactionStatus,

    #[serde(rename = "server_number")]
    pub server_id: Option<ServerId>,

    /// Keys authorized to access the rescue system via SSH.
    #[serde(
        rename = "authorized_key",
        deserialize_with = "crate::api::wrapper::deserialize_inner_vec"
    )]
    pub authorized_keys: Vec<InitialProductSshKey>,

    /// Host keys associated with the product.
    #[serde(
        rename = "host_key",
        deserialize_with = "crate::api::wrapper::deserialize_inner_vec"
    )]
    pub host_keys: Vec<HostKey>,

    /// Optional comment associated with the purchase.
    pub comment: Option<String>,

    /// Summary of the purchased product configuration.
    pub product: PurchasedProduct,

    /// Addons purchased for this product.
    pub addons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "in process")]
    InProcess,
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Transaction ID, e.g. "B20150121-344957-251478".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransactionId(pub String);

impl From<String> for TransactionId {
    fn from(value: String) -> Self {
        TransactionId(value)
    }
}

impl From<&str> for TransactionId {
    fn from(value: &str) -> Self {
        TransactionId(value.to_string())
    }
}

impl From<TransactionId> for String {
    fn from(value: TransactionId) -> Self {
        value.0
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for TransactionId {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// SSH Public Key provided as an authorized key when purchasing a server.
///
/// This is just key metadata, it does not contain the key itself. To retrieve the key, see [`AsyncRobot::get_ssh_key`](crate::AsyncRobot::get_ssh_key).
///
/// Similar to the [`SshKeyReference`](crate::api::keys::SshKeyReference), but does not return the time at which the key was created.
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct InitialProductSshKey {
    /// Unique name for the key.
    pub name: String,

    /// Fingerprint of the public key.
    pub fingerprint: String,

    /// Key algorithm (ED25519, RSA)
    #[serde(rename = "type")]
    pub algorithm: String,

    /// Key bit size.
    #[serde(rename = "size")]
    pub bits: u16,
}

/// SSH Host Key
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct HostKey {
    /// Fingerprint of the public key.
    pub fingerprint: String,

    /// Key algorithm (ED25519, RSA)
    #[serde(rename = "type")]
    pub algorithm: String,

    /// Key bit size.
    #[serde(rename = "size")]
    pub bits: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarketProduct {
    /// Unique identifier for this market product.
    pub id: MarketProductId,

    /// Human-readable name for this product.
    pub name: String,

    /// Human-readable list of features for this product.
    pub description: Vec<String>,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    #[serde(rename = "traffic", deserialize_with = "crate::conversion::traffic")]
    pub traffic_limit: Option<ByteSize>,

    /// Distribution selected for the purchased product.
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,

    /// Language selected for the product.
    #[serde(rename = "lang")]
    pub languages: Vec<String>,

    /// Datacenter of the purchased product.
    pub datacenter: Option<String>,

    /// Model name of the CPU
    pub cpu: String,

    /// CPU benchmark score.
    pub cpu_benchmark: u32,

    #[serde(deserialize_with = "crate::conversion::gb")]
    pub memory_size: ByteSize,

    #[serde(deserialize_with = "crate::conversion::gb")]
    pub hdd_size: ByteSize,

    pub hdd_text: String,

    pub hdd_count: u8,
}

/// Unique Market Product ID.
///
/// Uniquely identifies a product on the Hetzner (auction) market.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarketProductId(pub u32);

impl From<u32> for MarketProductId {
    fn from(value: u32) -> Self {
        MarketProductId(value)
    }
}

impl From<MarketProductId> for u32 {
    fn from(value: MarketProductId) -> Self {
        value.0
    }
}

impl Display for MarketProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<u32> for MarketProductId {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::api::{ordering::Transaction, wrapper::List};

    #[test]
    #[traced_test]
    fn deserialize_transactions() {
        let example_data = r#"
        [
            {
                "transaction":{
                "id":"B20150121-344957-251478",
                "date":"2015-01-21T12:30:43+01:00",
                "status":"in process",
                "server_number":null,
                "server_ip":null,
                "authorized_key":[
            
                ],
                "host_key":[
            
                ],
                "comment":null,
                "product":{
                    "id":"VX6",
                    "name":"vServer VX6",
                    "description":[
                    "Single-Core CPU",
                    "1 GB RAM",
                    "25 GB HDD",
                    "No telephone support"
                    ],
                    "traffic":"2 TB",
                    "dist":"Rescue system",
                    "@deprecated arch":"64",
                    "lang":"en",
                    "location":null
                },
                "addons":[
                    "primary_ipv4"
                ]
                }
            },
            {
                "transaction":{
                "id":"B20150121-344958-251479",
                "date":"2015-01-21T12:54:01+01:00",
                "status":"ready",
                "server_number":107239,
                "server_ip":"188.40.1.1",
                "authorized_key":[
                    {
                        "key":{
                            "name":"key1",
                            "fingerprint":"15:28:b0:03:95:f0:77:b3:10:56:15:6b:77:22:a5:bb",
                            "type":"ED25519",
                            "size":256
                        }
                    }
                ],
                "host_key":[
                    {
                        "key":{
                            "fingerprint":"c1:e4:08:73:dd:f7:e9:d1:94:ab:e9:0f:28:b2:d2:ed",
                            "type":"DSA",
                            "size":1024
                        }
                    }
                ],
                "comment":null,
                "product":{
                    "id":"EX40",
                    "name":"Dedicated Root Server EX40",
                    "description":[
                    "Intel\u00ae Core\u2122 i7-4770 Quad-Core Haswell",
                    "32 GB DDR3 RAM",
                    "2 x 2 TB SATA 6 Gb\/s Enterprise HDD; 7200 rpm(Software-RAID 1)",
                    "1 Gbit\/s bandwidth"
                    ],
                    "traffic":"30 TB",
                    "dist":"Debian 7.7 minimal",
                    "@deprecated arch":"64",
                    "lang":"en",
                    "location":"FSN1"
                },
                "addons":[
            
                ]
                }
            }
            ]"#;
        let transactions: List<Transaction> = serde_json::from_str(example_data).unwrap();

        info!("{transactions:#?}");
    }
}
