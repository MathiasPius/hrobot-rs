use std::{collections::HashMap, fmt::Display, net::IpAddr};

use bytesize::ByteSize;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use time::{OffsetDateTime, PrimitiveDateTime};
use time_tz::PrimitiveDateTimeExt;

use crate::{api::server::ServerId, urlencode::UrlEncode};

/// Describes a product available for purchase.
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

/// Describes a product purchase, as listed in a [`ProductTransaction`].
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

/// Describes a purchased market (auction) product, as described in a [`MarketTransaction`]
#[derive(Debug, Clone, Deserialize)]
pub struct PurchasedMarketProduct {
    /// Unique identifier for this product type.
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
    pub distribution: String,

    /// Language selected for the product.
    #[serde(rename = "lang")]
    pub language: String,

    /// Location of the purchased product.
    #[serde(rename = "location")]
    pub location: Option<Location>,

    /// Model name of the CPU
    pub cpu: String,

    /// CPU benchmark score.
    pub cpu_benchmark: u32,

    /// Total amount of memory installed in the server.
    #[serde(deserialize_with = "crate::conversion::gb")]
    pub memory_size: ByteSize,

    /// Primary hard drive capacity.
    ///
    /// Note that this only covers the capacity of the primary
    /// hard drive type, not the total capacity across all drives.
    ///
    /// In a server with the following configuration for example:
    /// * 6x SSD U.2 NVMe 3,84 TB Datacenter
    /// * 2x SSD SATA 3,84 TB Datacenter
    ///
    /// The HDD size will be 3.84TB, and [`MarketProduct::primary_hdd_count`] will be 6, not 8.
    #[serde(rename = "hdd_size", deserialize_with = "crate::conversion::gb")]
    pub primary_hdd_size: ByteSize,

    /// Human-readable summary of installed hardware/features, such as
    /// hard drive listing, ECC, INIC, etc.
    #[serde(rename = "hdd_text")]
    pub features: String,

    /// Primary hard drive count.
    ///
    /// Note that this only covers the installed count of the primary
    /// hard drive type, not the total number of drives.
    ///
    /// In a server with the following configuration for example:
    /// * 6x SSD U.2 NVMe 3,84 TB Datacenter
    /// * 2x SSD SATA 3,84 TB Datacenter
    ///
    /// The HDD size will be 3.84TB, and [`MarketProduct::primary_hdd_count`] will be 6, not 8.
    #[serde(rename = "hdd_count")]
    pub primary_hdd_count: u8,
}

fn location_prices<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<Location, LocationPrice>, D::Error> {
    let prices = Vec::<SingleLocationPrice>::deserialize(deserializer)?;

    Ok(prices
        .into_iter()
        .map(
            |SingleLocationPrice {
                 location,
                 recurring: monthly,
                 setup,
             }| {
                (
                    location,
                    LocationPrice {
                        recurring: monthly,
                        setup,
                    },
                )
            },
        )
        .collect())
}

/// Price information for a single location.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SingleLocationPrice {
    /// Location this price applies to.
    pub location: Location,
    /// Monthly price.
    #[serde(rename = "price")]
    pub recurring: RecurringPrice,
    /// One-time setup fee.
    #[serde(rename = "price_setup")]
    pub setup: SetupPrice,
}

/// Price (both setup and recurring) for a single location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationPrice {
    /// Monthly price in euros.
    pub recurring: RecurringPrice,
    /// One-time setup price in euros.
    pub setup: SetupPrice,
}

/// A recurring price point, both excluding and including VAT.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RecurringPrice {
    /// Monthly price excluding VAT.
    pub net: Decimal,
    /// Monthly price including VAT.
    pub gross: Decimal,
    /// Hourly price excluding VAT.
    pub hourly_net: Decimal,
    /// Hourly price including VAT.
    pub hourly_gross: Decimal,
}

/// A one-time setup price point, both excluding and including VAT.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SetupPrice {
    /// Monthly price excluding VAT.
    pub net: Decimal,
    /// Monthly price including VAT.
    pub gross: Decimal,
}

/// Describes an addon which can be purchased.
#[derive(Debug, Clone, Deserialize)]
pub struct Addon {
    /// Unique identifier for this addon.
    pub id: AddonId,

    /// Human-readable name for the addon.
    pub name: String,

    /// Location where this addon is available, or `None` is everywhere.
    pub location: Option<Location>,

    /// Minimum number available.
    pub min: u32,

    /// Maximum number available.
    pub max: u32,

    /// Prices for this addon in each location.
    #[serde(deserialize_with = "location_prices")]
    pub prices: HashMap<Location, LocationPrice>,
}

/// Describes an addon available for purchase for a specific server.
#[derive(Debug, Clone, Deserialize)]
pub struct AvailableAddon {
    /// Unique ID for this addon.
    pub id: AddonId,

    /// Human-readable name for the addon.
    pub name: String,

    /// Type of addon.
    pub r#type: String,

    /// Price for this addon in the target server's location.
    pub price: SingleLocationPrice,
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

/// Describes the purchase of a single standard hetzner product.
#[derive(Debug, Clone, Deserialize)]
pub struct ProductTransaction {
    /// Unique transaction ID.
    pub id: TransactionId,

    /// Timestamp for the purchase.
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,

    /// Status of the transaction.
    pub status: TransactionStatus,

    /// Server ID of the purchased product.
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

/// Status of the transaction.
#[derive(Debug, Clone, Deserialize)]
pub enum TransactionStatus {
    /// Transaction completed.
    #[serde(rename = "ready")]
    Ready,

    /// Transaction is still getting processed.
    #[serde(rename = "in process")]
    InProcess,

    /// Transaction has been cancelled.
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

/// Describes the purchase of a single Hetzner market (auction) server.
#[derive(Debug, Clone, Deserialize)]
pub struct MarketTransaction {
    /// Unique transaction ID.
    pub id: MarketTransactionId,

    /// Timestamp for the purchase.
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,

    /// Status of the transaction.
    pub status: TransactionStatus,

    /// Server ID of the purchased server.
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
    pub product: PurchasedMarketProduct,
}

/// Market Transaction ID, e.g. "B20150121-344957-251478".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MarketTransactionId(pub String);

impl From<String> for MarketTransactionId {
    fn from(value: String) -> Self {
        MarketTransactionId(value)
    }
}

impl From<&str> for MarketTransactionId {
    fn from(value: &str) -> Self {
        MarketTransactionId(value.to_string())
    }
}

impl From<MarketTransactionId> for String {
    fn from(value: MarketTransactionId) -> Self {
        value.0
    }
}

impl Display for MarketTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for MarketTransactionId {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Addon Transaction ID, e.g. "B20150121-344957-251478".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddonTransactionId(pub String);

impl From<String> for AddonTransactionId {
    fn from(value: String) -> Self {
        AddonTransactionId(value)
    }
}

impl From<&str> for AddonTransactionId {
    fn from(value: &str) -> Self {
        AddonTransactionId(value.to_string())
    }
}

impl From<AddonTransactionId> for String {
    fn from(value: AddonTransactionId) -> Self {
        value.0
    }
}

impl Display for AddonTransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for AddonTransactionId {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Describes the purchase of a single addon.
#[derive(Debug, Clone, Deserialize)]
pub struct AddonTransaction {
    /// Unique transacton ID.
    pub id: AddonTransactionId,

    /// Timestamp for the purchase.
    #[serde(with = "time::serde::rfc3339")]
    pub date: OffsetDateTime,

    /// Status of the transaction.
    pub status: TransactionStatus,

    /// Server ID which the purchased addon applies to.
    #[serde(rename = "server_number")]
    pub server_id: ServerId,

    /// Summary of the purchased addon.
    pub product: PurchasedAddon,

    /// Resources associated with this addon purchase.
    pub resources: Vec<Resource>,
}

/// Resource associated with an addon purchase.
#[derive(Debug, Clone, Deserialize)]
pub struct Resource {
    /// Indicates the type of the resource. e.g. `subnet`
    pub r#type: String,
    /// The ID of the resource. e.g. `10.0.0.0`
    pub id: String,
}

/// Describes a purchased addon as it appears in an [`AddonTransaction`]
#[derive(Debug, Clone, Deserialize)]
pub struct PurchasedAddon {
    /// Unique identifier for this product type.
    pub id: AddonId,

    /// Human-readable name for this product.
    pub name: String,

    /// Price the addon was purchased for.
    pub price: SingleLocationPrice,
}

/// Unique addon ID.
///
/// Uniquely identifies an addon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddonId(pub String);

impl From<String> for AddonId {
    fn from(value: String) -> Self {
        AddonId(value)
    }
}

impl From<&str> for AddonId {
    fn from(value: &str) -> Self {
        AddonId(value.to_string())
    }
}

impl From<AddonId> for String {
    fn from(value: AddonId) -> Self {
        value.0
    }
}

impl Display for AddonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<str> for AddonId {
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
struct InternalMarketProduct {
    pub id: MarketProductId,
    pub name: String,
    pub description: Vec<String>,
    #[serde(rename = "traffic", deserialize_with = "crate::conversion::traffic")]
    pub traffic_limit: Option<ByteSize>,
    #[serde(rename = "dist")]
    pub distributions: Vec<String>,
    #[serde(rename = "lang")]
    pub languages: Vec<String>,
    pub datacenter: Option<String>,
    pub cpu: String,
    pub cpu_benchmark: u32,
    #[serde(deserialize_with = "crate::conversion::gb")]
    pub memory_size: ByteSize,
    #[serde(deserialize_with = "crate::conversion::gb")]
    pub hdd_size: ByteSize,
    pub hdd_text: String,
    pub hdd_count: u8,
    pub price: Decimal,
    pub price_vat: Decimal,
    pub price_setup: Decimal,
    pub price_hourly: Decimal,
    pub price_hourly_vat: Decimal,
    pub price_setup_vat: Decimal,
    pub fixed_price: bool,
    pub next_reduce: i64,
    pub next_reduce_date: String,
    pub orderable_addons: Vec<Addon>,
}

/// Describes a Hetzner market (auction) product.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "InternalMarketProduct")]
pub struct MarketProduct {
    /// Unique identifier for this market product.
    pub id: MarketProductId,

    /// Human-readable name for this product.
    pub name: String,

    /// Human-readable list of features for this product.
    pub description: Vec<String>,

    /// Monthly traffic limitation if any, e.g. `5 TB`.
    pub traffic_limit: Option<ByteSize>,

    /// Distribution selected for the purchased product.
    pub distributions: Vec<String>,

    /// Language selected for the product.
    pub languages: Vec<String>,

    /// Datacenter of the purchased product.
    pub datacenter: Option<String>,

    /// Model name of the CPU
    pub cpu: String,

    /// CPU benchmark score.
    pub cpu_benchmark: u32,

    /// Total amount of memory installed in the server.
    pub memory_size: ByteSize,

    /// Primary hard drive capacity.
    ///
    /// Note that this only covers the capacity of the primary
    /// hard drive type, not the total capacity across all drives.
    ///
    /// In a server with the following configuration for example:
    /// * 6x SSD U.2 NVMe 3,84 TB Datacenter
    /// * 2x SSD SATA 3,84 TB Datacenter
    ///
    /// The HDD size will be 3.84TB, and [`MarketProduct::primary_hdd_count`] will be 6, not 8.
    pub primary_hdd_size: ByteSize,

    /// Human-readable summary of installed hardware/features, such as
    /// hard drive listing, ECC, INIC, etc.
    pub features: String,

    /// Primary hard drive count.
    ///
    /// Note that this only covers the installed count of the primary
    /// hard drive type, not the total number of drives.
    ///
    /// In a server with the following configuration for example:
    /// * 6x SSD U.2 NVMe 3,84 TB Datacenter
    /// * 2x SSD SATA 3,84 TB Datacenter
    ///
    /// The HDD size will be 3.84TB, and [`MarketProduct::primary_hdd_count`] will be 6, not 8.
    pub primary_hdd_count: u8,

    /// Price of the market product.
    pub price: LocationPrice,

    /// Indicates that the lowest price point has been reached, and won't be lowered further.
    pub fixed_price: bool,

    /// Time until the price of the product is reduced.
    pub next_reduce_in: std::time::Duration,

    /// Timestamp indicating the time at which the product price will be further reduced.
    pub next_reduce_at: Option<OffsetDateTime>,

    /// List of available addons for the product.
    pub orderable_addons: Vec<Addon>,
}

impl From<InternalMarketProduct> for MarketProduct {
    fn from(value: InternalMarketProduct) -> Self {
        MarketProduct {
            id: value.id,
            name: value.name,
            description: value.description,
            traffic_limit: value.traffic_limit,
            distributions: value.distributions,
            languages: value.languages,
            datacenter: value.datacenter,
            cpu: value.cpu,
            cpu_benchmark: value.cpu_benchmark,
            memory_size: value.memory_size,
            primary_hdd_size: value.hdd_size,
            features: value.hdd_text,
            primary_hdd_count: value.hdd_count,
            fixed_price: value.fixed_price,
            price: LocationPrice {
                recurring: RecurringPrice {
                    net: value.price,
                    gross: value.price_vat,
                    hourly_net: value.price_hourly,
                    hourly_gross: value.price_hourly_vat,
                },
                setup: SetupPrice {
                    net: value.price_setup,
                    gross: value.price_setup_vat,
                },
            },
            next_reduce_in: std::time::Duration::from_secs(value.next_reduce.unsigned_abs()),
            next_reduce_at: PrimitiveDateTime::parse(
                &value.next_reduce_date,
                &time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
            )
            .ok()
            .and_then(|time| {
                time.assume_timezone(time_tz::timezones::db::europe::BERLIN)
                    .take()
            }),
            orderable_addons: value.orderable_addons,
        }
    }
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

/// Authorization method chosen for the purchase.
/// Only one can be selected.
#[derive(Debug, Clone)]
pub enum AuthorizationMethod {
    /// List of fingerprints corresponding to ssh keys already
    /// provisioned within the Hetzner Robot system.
    Keys(Vec<String>),
    /// Set a root password for the server upon provisioning.
    Password(String),
}

/// LetMeSpendMyMoneyAlready must be selected for any purchase order to
/// actually go through, otherwise the "test" flag will be set.
/// and the API will just simulate a purchase, returning a
/// "Cancelled" transaction.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum ImSeriousAboutSpendingMoney {
    /// This variant must be selected for any purchase order to
    /// actually go through, otherwise the "test" flag will be set.
    /// and the API will just simulate a purchase, returning a
    /// "Cancelled" transaction.
    LetMeSpendMyMoneyAlready,
    /// This variant will mark the purchase order as just a test,
    /// meaning the API will return a "Cancelled" transaction, and
    /// not actually go through with the purchase, only simulate it.
    #[default]
    NoThisIsJustATest,
}

/// Order for a standard Hetzner product, such as AX41.
///
/// Note: this is different from a [`MarketProductOrder`] which pertains
/// to purchase orders on the Hetzner auction market.
#[derive(Debug, Clone)]
pub struct ProductOrder {
    /// Id of the product to purchase.
    pub id: ProductId,
    /// Authorization method which should be enabled for the installed distribution.
    pub auth: AuthorizationMethod,
    /// Where this product should be provisioned.
    pub location: Location,
    /// Distribution to install on the target server.
    pub distribution: Option<String>,
    /// Chosen language for the installed distribution.
    pub language: Option<String>,
    /// Comment for the order. Note that comments require manual provisioning,
    /// which can increase the processing time for the purchase request.
    pub comment: Option<String>,
    /// Addons to order alongside this product.
    pub addons: Vec<AddonId>,

    /// LetMeSpendMyMoneyAlready must be selected for any purchase order to
    /// actually go through, otherwise the "test" flag will be set.
    /// and the API will just simulate a purchase, returning a
    /// "Cancelled" transaction.
    pub i_want_to_spend_money_to_purchase_a_server: ImSeriousAboutSpendingMoney,
}

impl UrlEncode for ProductOrder {
    fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
        f.set("product_id", &self.id);

        match &self.auth {
            AuthorizationMethod::Keys(keys) => {
                for key in keys {
                    f.set("authorized_key[]", key)
                }
            }
            AuthorizationMethod::Password(password) => {
                f.set("password", password);
            }
        }

        f.set("location", &self.location);

        if let Some(dist) = &self.distribution {
            f.set("dist", dist);
        }

        if let Some(lang) = &self.language {
            f.set("lang", lang);
        }

        if let Some(comment) = &self.comment {
            f.set("comment", comment);
        }

        for addon in &self.addons {
            f.set("addon[]", addon);
        }

        if self.i_want_to_spend_money_to_purchase_a_server
            == ImSeriousAboutSpendingMoney::LetMeSpendMyMoneyAlready
        {
            f.set("test", "false")
        } else {
            f.set("test", "true")
        }
    }
}

/// Hetzner Auction market order.
///
/// Note: this is distinct from the [`ProductOrder`] which pertains to
/// standard Hetzner products such as AX41.
#[derive(Debug, Clone)]
pub struct MarketProductOrder {
    /// Auction server ID.
    pub id: MarketProductId,

    /// Authorization method which should be enabled for the installed distribution.
    pub auth: AuthorizationMethod,

    /// Distribution to install on the purchased server.
    pub distribution: Option<String>,

    /// Chosen language for the selected distribution.
    pub language: Option<String>,

    /// Comment for the order. Note that comments require manual provisioning,
    /// which can increase the processing time for the purchase request.
    pub comment: Option<String>,

    /// Addons to purchase alongside this server.
    pub addons: Vec<AddonId>,

    /// LetMeSpendMyMoneyAlready must be selected for any purchase order to
    /// actually go through, otherwise the "test" flag will be set.
    /// and the API will just simulate a purchase, returning a
    /// "Cancelled" transaction.
    pub i_want_to_spend_money_to_purchase_a_server: ImSeriousAboutSpendingMoney,
}

impl UrlEncode for MarketProductOrder {
    fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
        f.set("product_id", self.id);

        match &self.auth {
            AuthorizationMethod::Keys(keys) => {
                for key in keys {
                    f.set("authorized_key[]", key)
                }
            }
            AuthorizationMethod::Password(password) => {
                f.set("password", password);
            }
        }

        if let Some(dist) = &self.distribution {
            f.set("dist", dist);
        }

        if let Some(lang) = &self.language {
            f.set("lang", lang);
        }

        if let Some(comment) = &self.comment {
            f.set("comment", comment);
        }

        for addon in &self.addons {
            f.set("addon[]", addon);
        }

        if self.i_want_to_spend_money_to_purchase_a_server
            == ImSeriousAboutSpendingMoney::LetMeSpendMyMoneyAlready
        {
            f.set("test", "false")
        } else {
            f.set("test", "true")
        }
    }
}

/// Addon purchase order.
#[derive(Debug, Clone)]
pub struct AddonOrder {
    /// Unique ID of the addon to be purchased.
    pub id: AddonId,

    /// Server ID which this addon applies to.
    pub server: ServerId,

    /// RIPE reason: mandatory for addon types "ip_ipv4", "subnet_ipv4"
    /// and "failover_subnet_ipv4"
    pub reason: Option<String>,

    /// Routing target for subnets: usable for addon type "subnet_ipv4"
    /// (Optional: default is the server's primary IP address)
    pub gateway: Option<IpAddr>,

    /// LetMeSpendMyMoneyAlready must be selected for any purchase order to
    /// actually go through, otherwise the "test" flag will be set.
    /// and the API will just simulate a purchase, returning a
    /// "Cancelled" transaction.
    pub i_want_to_spend_money_to_purchase_an_addon: ImSeriousAboutSpendingMoney,
}

impl UrlEncode for AddonOrder {
    fn encode_into(&self, mut f: crate::urlencode::UrlEncodingBuffer<'_>) {
        f.set("product_id", &self.id);
        f.set("server_number", self.server);

        if let Some(reason) = &self.reason {
            f.set("reason", reason);
        }

        if let Some(gateway) = &self.gateway {
            f.set("gateway", gateway);
        }

        if self.i_want_to_spend_money_to_purchase_an_addon
            == ImSeriousAboutSpendingMoney::LetMeSpendMyMoneyAlready
        {
            f.set("test", "false")
        } else {
            f.set("test", "true")
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::{
        api::{
            ordering::{
                AddonId, AddonTransaction, AuthorizationMethod, AvailableAddon,
                ImSeriousAboutSpendingMoney, MarketProductId, MarketTransaction,
                ProductTransaction,
            },
            wrapper::List,
        },
        urlencode::UrlEncode,
    };

    use super::MarketProductOrder;

    #[test]
    #[traced_test]
    fn test_serialize_market_product_order() {
        let a = MarketProductOrder {
            id: MarketProductId(100),
            auth: AuthorizationMethod::Keys(vec![
                String::from("15:28:b0:03:95:f0:77:b3:10:56:15:6b:77:22:a5:aa"),
                String::from("15:28:b0:03:95:f0:77:b3:10:56:15:6b:77:22:a5:bb"),
            ]),
            distribution: Some("Rescue System".to_string()),
            language: Some("en".to_string()),
            addons: vec![AddonId::from("primary_ipv4")],
            comment: None,
            i_want_to_spend_money_to_purchase_a_server:
                ImSeriousAboutSpendingMoney::NoThisIsJustATest,
        };

        info!("{}", a.encode());
    }

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
        let transactions: List<ProductTransaction> = serde_json::from_str(example_data).unwrap();

        info!("{transactions:#?}");
    }

    #[test]
    #[traced_test]
    fn test_deserialize_market_transaction() {
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
                        "id":283693,
                        "name":"SB110",
                        "description":[
                        "Intel Core i7 980x",
                        "6x RAM 4096 MB DDR3",
                        "2x HDD 1,5 TB SATA",
                        "2x SSD 120 GB SATA"
                        ],
                        "traffic":"20 TB",
                        "dist":"Rescue system",
                        "@deprecated arch":"64",
                        "lang":"en",
                        "cpu":"Intel Core i7 980x",
                        "cpu_benchmark":8944,
                        "memory_size":24,
                        "hdd_size":1536,
                        "hdd_text":"ENT.HDD ECC INIC",
                        "hdd_count":2,
                        "datacenter":"FSN1-DC5",
                        "network_speed":"100 Mbit\/s",
                        "fixed_price":true,
                        "next_reduce":0,
                        "next_reduce_date":"2018-05-01 12:22:00"
                    }
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
                        "id":277254,
                        "name":"SB114",
                        "description":[
                        "Intel Core i7 950",
                        "6x RAM 2048 MB DDR3",
                        "7x HDD 1,5 TB SATA"
                        ],
                        "traffic":"20 TB",
                        "dist":"Rescue system",
                        "@deprecated arch":"64",
                        "lang":"en",
                        "cpu":"Intel Core i7 950",
                        "cpu_benchmark":5682,
                        "memory_size":12,
                        "hdd_size":1536,
                        "hdd_text":"ENT.HDD ECC INIC",
                        "hdd_count":7,
                        "datacenter":"FSN1-DC5",
                        "network_speed":"100 Mbit\/s",
                        "fixed_price":true,
                        "next_reduce":0,
                        "next_reduce_date":"2018-05-01 12:22:00"
                    }
                    }
                }
            ]"#;

        let transactions: List<MarketTransaction> = serde_json::from_str(example_data).unwrap();

        info!("{transactions:#?}");
    }

    #[test]
    #[traced_test]
    fn test_deserialize_addon_transactions() {
        // This is the example shown in the API documentation:
        // <https://robot.hetzner.com/doc/webservice/en.html#get-order-server_market-transaction-id>
        let example_data = r#"
            [
                {
                    "transaction":{
                        "id":"B20220210-1843193-S33055",
                        "date":"2022-02-10T12:20:11+01:00",
                        "status":"in process",
                        "server_number":123,
                        "product":{
                        "id":"failover_subnet_ipv4_29",
                        "name":"Failover subnet \/29",
                        "price":{
                            "location":"NBG1",
                            "price":{
                                "net":"15.1261",
                                "gross":"15.1261",
                                "hourly_net":"0.0242",
                                "hourly_gross":"0.0242"
                            },
                            "price_setup":{
                                "net":"152.0000",
                                "gross":"152.0000"
                            }
                        }
                        },
                        "resources":[
                
                        ]
                    }
                },
                {
                    "transaction":{
                        "id":"B20220210-1843192-S33051",
                        "date":"2022-02-10T11:20:13+01:00",
                        "status":"ready",
                        "server_number":123,
                        "product":{
                        "id":"failover_subnet_ipv4_29",
                        "name":"Failover subnet \/29",
                        "price":{
                            "location":"NBG1",
                            "price":{
                                "net":"15.1261",
                                "gross":"15.1261",
                                "hourly_net":"0.0242",
                                "hourly_gross":"0.0242"
                            },
                            "price_setup":{
                                "net":"152.0000",
                                "gross":"152.0000"
                            }
                        }
                        },
                        "resources":[
                        {
                            "type":"subnet",
                            "id":"10.0.0.0"
                        }
                        ]
                    }
                }
            ]
          "#;

        let transactions: List<AddonTransaction> = serde_json::from_str(example_data).unwrap();

        info!("{transactions:#?}");
    }

    #[test]
    #[traced_test]
    fn test_deserialize_available_addons() {
        // This is the example used in the API documentation
        // <https://robot.hetzner.com/doc/webservice/en.html#get-order-server_addon-server-number-product>
        let example_data = r#"
          [
            {
              "product":{
                "id":"additional_ipv4",
                "name":"Additional IP address",
                "type":"ip_ipv4",
                "price":{
                  "location":"NBG1",
                  "price":{
                    "net":"0.8403",
                    "gross":"0.8403",
                    "hourly_net":"0.0014",
                    "hourly_gross":"0.0014"
                  },
                  "price_setup":{
                    "net":"19.0000",
                    "gross":"19.0000"
                  }
                }
              }
            },
            {
              "product":{
                "id":"subnet_ipv4_29",
                "name":"Additional subnet \/29 (monthly charge)",
                "type":"subnet_ipv4",
                "price":{
                  "location":"NBG1",
                  "price":{
                    "net":"6.7227",
                    "gross":"6.7227",
                    "hourly_net":"0.0108",
                    "hourly_gross":"0.0108"
                  },
                  "price_setup":{
                    "net":"152.0000",
                    "gross":"152.0000"
                  }
                }
              }
            }
          ]"#;

        let data: List<AvailableAddon> = serde_json::from_str(&example_data).unwrap();

        info!("{data:#?}");
    }
}
