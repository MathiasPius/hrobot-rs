use std::{collections::HashMap, fmt::Display};

use bytesize::ByteSize;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

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
    #[serde(rename = "location")]
    pub locations: Vec<Location>,

    /// Prices for this product in each location
    #[serde(deserialize_with = "location_prices")]
    pub prices: HashMap<Location, LocationPrice>,

    /// Addons which can be purchased for this product.
    pub orderable_addons: Vec<Addon>,
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
