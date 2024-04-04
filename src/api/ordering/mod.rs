//! Server and addon purchasing structs and implementation.

mod models;
use std::ops::RangeBounds;

pub use models::*;
use rust_decimal::prelude::Zero;
use serde::Serialize;

use crate::{error::Error, urlencode::UrlEncode, AsyncRobot};

use super::{
    server::ServerId,
    wrapper::{List, Single},
    UnauthenticatedRequest,
};

fn list_products(
    monthly_price: impl RangeBounds<u32>,
    setup_price: impl RangeBounds<u32>,
    location: Option<&Location>,
) -> Result<UnauthenticatedRequest<List<Product>>, serde_html_form::ser::Error> {
    fn is_max(value: &u32) -> bool {
        *value == u32::MAX
    }

    #[derive(Debug, Serialize)]
    struct ProductSearch<'a> {
        #[serde(skip_serializing_if = "u32::is_zero")]
        min_price: u32,
        #[serde(skip_serializing_if = "is_max")]
        max_price: u32,
        #[serde(skip_serializing_if = "u32::is_zero")]
        min_price_setup: u32,
        #[serde(skip_serializing_if = "is_max")]
        max_price_setup: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<&'a Location>,
    }

    // Transform a RangeBounds-implementing object into a
    // reasonable integer approximation.
    fn capped(range: &impl RangeBounds<u32>) -> (u32, u32) {
        (
            match range.start_bound() {
                std::ops::Bound::Included(n) => *n,
                std::ops::Bound::Excluded(n) => *n + 1,
                std::ops::Bound::Unbounded => 0,
            },
            match range.end_bound() {
                std::ops::Bound::Included(n) => *n,
                std::ops::Bound::Excluded(n) => std::cmp::max(1, *n) - 1,
                std::ops::Bound::Unbounded => u32::MAX,
            },
        )
    }

    let search = ProductSearch {
        min_price: capped(&monthly_price).0,
        max_price: capped(&monthly_price).1,
        min_price_setup: capped(&setup_price).0,
        max_price_setup: capped(&setup_price).1,
        location,
    };

    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/product")
        .with_query_params(search)
}

fn get_product(id: &ProductId) -> UnauthenticatedRequest<Single<Product>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server/product/{id}"
    ))
}

fn list_product_tranactions() -> UnauthenticatedRequest<List<ProductTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/transaction")
}

fn get_product_transaction(
    id: &TransactionId,
) -> UnauthenticatedRequest<Single<ProductTransaction>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server/transaction/{id}"
    ))
}

fn list_market_products() -> UnauthenticatedRequest<List<MarketProduct>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server_market/product/")
}

fn get_market_product(id: &MarketProductId) -> UnauthenticatedRequest<Single<MarketProduct>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server_market/product/{id}"
    ))
}

fn list_market_product_transactions() -> UnauthenticatedRequest<List<MarketTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server_market/transaction")
}

fn get_market_product_transaction(
    id: &MarketTransactionId,
) -> UnauthenticatedRequest<Single<MarketTransaction>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server_market/transaction/{id}"
    ))
}

fn list_available_addons(id: ServerId) -> UnauthenticatedRequest<List<AvailableAddon>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server_addon/{id}/product"
    ))
}

fn list_addon_transactions() -> UnauthenticatedRequest<List<AddonTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server_addon/transaction")
}

fn get_addon_transaction(
    id: &AddonTransactionId,
) -> UnauthenticatedRequest<Single<AddonTransaction>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server_addon/transaction/{id}"
    ))
}

fn place_market_purchase_order(
    order: MarketProductOrder,
) -> UnauthenticatedRequest<Single<MarketTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server_market/transaction")
        .with_method("POST")
        .with_serialized_body(order.encode())
}

fn place_purchase_order(order: ProductOrder) -> UnauthenticatedRequest<Single<ProductTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/transaction")
        .with_method("POST")
        .with_serialized_body(order.encode())
}

fn place_addon_purchase_order(
    order: AddonOrder,
) -> UnauthenticatedRequest<Single<AddonTransaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server_addon/transaction")
        .with_method("POST")
        .with_serialized_body(order.encode())
}

impl AsyncRobot {
    /// List all available products.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::Location;
    /// # use hrobot::rust_decimal::Decimal;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for product in robot.list_products(
    ///     30..50,
    ///     ..0,
    ///     Some(&Location::from("FSN1")),
    /// ).await.unwrap() {
    ///     println!("{}: {}", product.id, product.name);
    /// }
    /// # }
    /// ```
    pub async fn list_products(
        &self,
        monthly_price: impl RangeBounds<u32>,
        setup_price: impl RangeBounds<u32>,
        location: Option<&Location>,
    ) -> Result<Vec<Product>, Error> {
        Ok(self
            .go(list_products(monthly_price, setup_price, location)?)
            .await?
            .0)
    }

    /// Get description of a single product.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::ProductId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let product = robot.get_product(
    ///     &ProductId::from("EX44")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_product(&self, id: &ProductId) -> Result<Product, Error> {
        Ok(self.go(get_product(id)).await?.0)
    }

    /// Purchase a standard server product.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::{
    /// #   AddonId, ProductId, AuthorizationMethod, ProductOrder,
    /// #   ImSeriousAboutSpendingMoney, Location,
    /// # };
    /// # use tracing::info;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let transaction = robot.place_product_order(
    ///     ProductOrder {
    ///         id: ProductId::from("EX41"),
    ///         auth: AuthorizationMethod::Keys(vec![
    ///             "15:28:b0:03:95:f0:77:b3:10:56:15:6b:77:22:a5:bb".to_string()
    ///         ]),
    ///         distribution: Some("Rescue system".to_string()),
    ///         language: Some("en".to_string()),
    ///         location: Location::from("FSN1"),
    ///         addons: vec![AddonId::from("primary_ipv4")],
    ///         comment: None,
    ///         // Don't forget to change this line, if you ACTUALLY want to make the purchase!
    ///         i_want_to_spend_money_to_purchase_a_server: ImSeriousAboutSpendingMoney::NoThisIsJustATest,
    ///     }
    /// ).await.unwrap();
    /// info!("{transaction:#?}");
    /// # }
    /// ```
    pub async fn place_product_order(
        &self,
        order: ProductOrder,
    ) -> Result<ProductTransaction, Error> {
        Ok(self.go(place_purchase_order(order)).await?.0)
    }

    /// List product transactions from the last 30 days.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for transaction in robot.list_recent_product_transactions().await.unwrap() {
    ///     println!("{}: {}", transaction.product.id, transaction.date);
    /// }
    /// # }
    /// ```
    pub async fn list_recent_product_transactions(&self) -> Result<Vec<ProductTransaction>, Error> {
        Ok(self.go(list_product_tranactions()).await?.0)
    }

    /// Get specific product transactions by ID.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::TransactionId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_product_transaction(
    ///     &TransactionId::from("B20150121-344958-251479")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_product_transaction(
        &self,
        transaction: &TransactionId,
    ) -> Result<ProductTransaction, Error> {
        Ok(self.go(get_product_transaction(transaction)).await?.0)
    }

    /// List market (auction) products.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for market_product in robot.list_market_products().await.unwrap() {
    ///     println!("{}: {}", market_product.id, market_product.name);
    /// }
    /// # }
    /// ```
    pub async fn list_market_products(&self) -> Result<Vec<MarketProduct>, Error> {
        Ok(self.go(list_market_products()).await?.0)
    }

    /// Get description of a single market (auction) product.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::MarketProductId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let product = robot.get_market_product(
    ///     &MarketProductId(2128654)
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_market_product(&self, id: &MarketProductId) -> Result<MarketProduct, Error> {
        Ok(self.go(get_market_product(id)).await?.0)
    }

    /// List market (auction) transactions from the last 30 days.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for transaction in robot.list_recent_market_transactions().await.unwrap() {
    ///     println!("{}: {}", transaction.product.id, transaction.date);
    /// }
    /// # }
    /// ```
    pub async fn list_recent_market_transactions(&self) -> Result<Vec<MarketTransaction>, Error> {
        Ok(self.go(list_market_product_transactions()).await?.0)
    }

    /// Get specific market (auction) transaction by ID.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::MarketTransactionId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_market_transaction(
    ///     &MarketTransactionId::from("B20150121-344958-251479")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_market_transaction(
        &self,
        transaction: &MarketTransactionId,
    ) -> Result<MarketTransaction, Error> {
        Ok(self
            .go(get_market_product_transaction(transaction))
            .await?
            .0)
    }

    /// Purchase a server product from the market.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::{
    /// #   AddonId, MarketProductId, AuthorizationMethod, MarketProductOrder,
    /// #   ImSeriousAboutSpendingMoney,
    /// # };
    /// # use tracing::info;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let transaction = robot.place_market_order(
    ///     MarketProductOrder {
    ///         id: MarketProductId(12345678),
    ///         auth: AuthorizationMethod::Keys(vec![
    ///             "15:28:b0:03:95:f0:77:b3:10:56:15:6b:77:22:a5:bb".to_string()
    ///         ]),
    ///         distribution: Some("Rescue system".to_string()),
    ///         language: Some("en".to_string()),
    ///         addons: vec![AddonId::from("primary_ipv4")],
    ///         comment: None,
    ///         // Don't forget to change this line, if you ACTUALLY want to make the purchase!
    ///         i_want_to_spend_money_to_purchase_a_server: ImSeriousAboutSpendingMoney::NoThisIsJustATest,
    ///     }
    /// ).await.unwrap();
    /// info!("{transaction:#?}");
    /// # }
    /// ```
    pub async fn place_market_order(
        &self,
        order: MarketProductOrder,
    ) -> Result<MarketTransaction, Error> {
        Ok(self.go(place_market_purchase_order(order)).await?.0)
    }

    /// List available addons for a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for addon in robot.list_available_addons(
    ///     ServerId(1234)
    /// ).await.unwrap() {
    ///     println!("{addon:#?}");
    /// }
    /// # }
    /// ```
    pub async fn list_available_addons(&self, id: ServerId) -> Result<Vec<AvailableAddon>, Error> {
        Ok(self.go(list_available_addons(id)).await?.0)
    }

    /// Purchase an addon for a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::{
    /// #   AddonId, AddonOrder,
    /// #   ImSeriousAboutSpendingMoney,
    /// # };
    /// # use hrobot::api::server::ServerId;
    /// # use tracing::info;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let transaction = robot.place_addon_order(
    ///     AddonOrder {
    ///         id: AddonId::from("additional_ipv4"),
    ///         server: ServerId(1234567),
    ///         reason: Some("VPS".to_string()),
    ///         gateway: Some("10.0.0.1".parse().unwrap()),
    ///         // Don't forget to change this line, if you ACTUALLY want to make the purchase!
    ///         i_want_to_spend_money_to_purchase_an_addon: ImSeriousAboutSpendingMoney::NoThisIsJustATest,
    ///     }
    /// ).await.unwrap();
    /// info!("{transaction:#?}");
    /// # }
    /// ```
    pub async fn place_addon_order(&self, order: AddonOrder) -> Result<AddonTransaction, Error> {
        Ok(self.go(place_addon_purchase_order(order)).await?.0)
    }

    /// List addon transactions from the last 30 days.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for transaction in robot.list_recent_product_transactions().await.unwrap() {
    ///     println!("{}: {}", transaction.product.id, transaction.date);
    /// }
    /// # }
    /// ```
    pub async fn list_recent_addon_transactions(&self) -> Result<Vec<AddonTransaction>, Error> {
        Ok(self.go(list_addon_transactions()).await?.0)
    }

    /// Get specific addon transaction by ID.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::AddonTransactionId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_addon_transaction(
    ///     &AddonTransactionId::from("B20150121-344958-251479")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_addon_transaction(
        &self,
        transaction: &AddonTransactionId,
    ) -> Result<AddonTransaction, Error> {
        Ok(self.go(get_addon_transaction(transaction)).await?.0)
    }
}
