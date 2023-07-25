use bytesize::ByteSize;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Vec<String>,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    #[serde(deserialize_with = "crate::conversion::traffic")]
    pub traffic: Option<ByteSize>,

    pub dist: Vec<String>,
    pub lang: Vec<String>,
    pub location: Vec<String>,
    pub prices: Vec<LocationPrice>,
    pub orderable_addons: Vec<Addon>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocationPrice {
    pub location: String,
    #[serde(rename = "price")]
    pub monthly: Price,
    #[serde(rename = "price_setup")]
    pub setup: Price,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    pub net: Decimal,
    pub gross: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Addon {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub min: u32,
    pub max: u32,
    pub prices: Vec<LocationPrice>,
}
