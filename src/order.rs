use serde::{Deserialize, Serialize};

use crate::{Error, SyncRobot};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Vec<String>,
    pub traffic: String,
    pub dist: Vec<String>,
    pub lang: Vec<String>,
    pub location: Vec<String>,
    pub prices: Vec<LocationPrice>,
    pub orderable_addons: Vec<OrderableAddon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketProduct {
    pub id: u32,
    pub name: String,
    pub description: Vec<String>,
    pub traffic: String,
    pub dist: Vec<String>,
    pub lang: Vec<String>,
    pub cpu: String,
    pub cpu_benchmark: u32,
    pub memory_size: u32,
    pub hdd_size: u32,
    pub hdd_text: String,
    pub hdd_count: u32,
    pub datacenter: String,
    pub network_speed: String,
    pub price: String,
    pub price_vat: String,
    pub price_setup: String,
    pub price_setup_vat: String,
    pub fixed_price: bool,
    pub next_reduce: i32,
    pub next_reduce_date: String,
    pub orderable_addons: Vec<OrderableAddon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderableAddon {
    pub id: String,
    pub name: String,
    // According to the API documentation each OrderableAddon
    // has a location field, but this is incorrect, instead each
    // AddonPrice has its own location tag. for per-location pricing.
    // pub location: Option<String>,
    pub min: u32,
    pub max: u32,
    pub prices: Vec<LocationPrice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationPrice {
    pub location: String,
    pub price: Price,
    pub price_setup: Price,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub net: String,
    pub gross: String,
}

#[derive(Debug, Deserialize)]
struct MarketProductResponse {
    pub product: MarketProduct,
}

impl From<MarketProductResponse> for MarketProduct {
    fn from(response: MarketProductResponse) -> Self {
        response.product
    }
}

#[derive(Debug, Deserialize)]
struct ProductResponse {
    pub product: Product,
}

impl From<ProductResponse> for Product {
    fn from(response: ProductResponse) -> Self {
        response.product
    }
}

/// Trait defining the ordering-related API endpoints of the Hetzner API. Implemented by [`Robot`]
pub trait OrderRobot {
    fn list_products(&self) -> Result<Vec<Product>, Error>;
    fn list_market_products(&self) -> Result<Vec<MarketProduct>, Error>;
}

impl<T> OrderRobot for T
where
    T: SyncRobot,
{
    fn list_products(&self) -> Result<Vec<Product>, Error> {
        self.get::<Vec<ProductResponse>>("/order/server/product")
            .map(|s| s.into_iter().map(Product::from).collect())
    }

    fn list_market_products(&self) -> Result<Vec<MarketProduct>, Error> {
        self.get::<Vec<MarketProductResponse>>("/order/server_market/product")
            .map(|s| s.into_iter().map(MarketProduct::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{OrderRobot, Robot};

    #[test]
    #[ignore]
    pub fn list_products() {
        let robot = Robot::default();
        let products = robot.list_products().unwrap();
        println!("{products:#?}");
        assert!(products.len() > 0);
    }

    #[test]
    #[ignore]
    pub fn list_market_products() {
        let robot = Robot::default();
        let products = robot.list_market_products().unwrap();
        assert!(products.len() > 0);
    }
}
