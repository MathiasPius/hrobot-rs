mod models;
use std::ops::RangeBounds;

pub use models::*;
use serde::Serialize;

use crate::{error::Error, AsyncRobot};

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
    #[derive(Debug, Serialize)]
    struct ProductSearch<'a> {
        min_price: u32,
        max_price: u32,
        min_price_setup: u32,
        max_price_setup: u32,
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
                std::ops::Bound::Unbounded => 9999,
            },
        )
    }

    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/product").with_body(
        ProductSearch {
            min_price: capped(&monthly_price).0,
            max_price: capped(&monthly_price).1,
            min_price_setup: capped(&setup_price).0,
            max_price_setup: capped(&setup_price).1,
            location,
        },
    )
}

fn get_product(id: &ProductId) -> UnauthenticatedRequest<Single<Product>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/order/server/product/{id}"
    ))
}

fn list_product_tranactions() -> UnauthenticatedRequest<List<Transaction>> {
    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/transaction")
}

fn get_product_transaction(id: &TransactionId) -> UnauthenticatedRequest<Single<Transaction>> {
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

impl AsyncRobot {
    /// List all available products.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::Location;
    /// # use hrobot::Decimal;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
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
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// let product = robot.get_product(
    ///     &ProductId::from("EX44")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_product(&self, id: &ProductId) -> Result<Product, Error> {
        Ok(self.go(get_product(id)).await?.0)
    }

    /// List product transactions from the last 30 days.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for transaction in robot.list_recent_product_transactions().await.unwrap() {
    ///     println!("{}: {}", transaction.product.id, transaction.date);
    /// }
    /// # }
    /// ```
    pub async fn list_recent_product_transactions(&self) -> Result<Vec<Transaction>, Error> {
        Ok(self.go(list_product_tranactions()).await?.0)
    }

    /// Get specific product transactions by ID.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::TransactionId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_product_transaction(
    ///     &TransactionId::from("B20150121-344958-251479")
    /// ).await.unwrap();
    /// # }
    /// ```
    pub async fn get_product_transaction(
        &self,
        transaction: &TransactionId,
    ) -> Result<Transaction, Error> {
        Ok(self.go(get_product_transaction(transaction)).await?.0)
    }

    /// List market (auction) products.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
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
    /// # dotenvy::dotenv().ok();
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
    /// # dotenvy::dotenv().ok();
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
    /// # dotenvy::dotenv().ok();
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

    /// List available addons for a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::ordering::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for addon in robot.list_available_addons(
    ///     ServerId(1234)
    /// ).await.unwrap() {
    ///     println!("{addon#:?}");
    /// }
    /// # }
    /// ```
    pub async fn list_available_addons(&self, id: ServerId) -> Result<Vec<AvailableAddon>, Error> {
        Ok(self.go(list_available_addons(id)).await?.0)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use tracing::info;
        use tracing_test::traced_test;

        use crate::{
            error::{ApiError, Error},
            AsyncRobot,
        };

        #[tokio::test]
        #[traced_test]
        async fn test_get_product_listing() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            for product in robot.list_products(.., .., None).await.unwrap() {
                info!("{product:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_single_product() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            if let Some(product) = robot.list_products(.., .., None).await.unwrap().first() {
                let product = robot.get_product(&product.id).await.unwrap();
                info!("{product:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_list_recent_product_transactions() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            for transaction in robot
                .list_recent_product_transactions()
                .await
                .or_else(|err| {
                    if matches!(err, Error::Api(ApiError::NotFound { .. })) {
                        Ok(vec![])
                    } else {
                        Err(err)
                    }
                })
                .unwrap()
            {
                info!("{transaction:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_recent_product_transactions() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            if let Some(transaction) = robot
                .list_recent_product_transactions()
                .await
                .or_else(|err| {
                    if matches!(err, Error::Api(ApiError::NotFound { .. })) {
                        Ok(vec![])
                    } else {
                        Err(err)
                    }
                })
                .unwrap()
                .first()
            {
                let transaction = robot
                    .get_product_transaction(&transaction.id)
                    .await
                    .unwrap();
                info!("{transaction:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_list_market_products() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            for product in robot.list_market_products().await.unwrap() {
                info!("{product:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_single_market_product() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            if let Some(product) = robot.list_market_products().await.unwrap().first() {
                let product = robot.get_market_product(&product.id).await.unwrap();
                info!("{product:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_list_recent_market_transactions() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            for transaction in robot
                .list_recent_market_transactions()
                .await
                .or_else(|err| {
                    if matches!(err, Error::Api(ApiError::NotFound { .. })) {
                        Ok(vec![])
                    } else {
                        Err(err)
                    }
                })
                .unwrap()
            {
                info!("{transaction:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_get_recent_market_transactions() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            if let Some(transaction) = robot
                .list_recent_market_transactions()
                .await
                .or_else(|err| {
                    if matches!(err, Error::Api(ApiError::NotFound { .. })) {
                        Ok(vec![])
                    } else {
                        Err(err)
                    }
                })
                .unwrap()
                .first()
            {
                let transaction = robot.get_market_transaction(&transaction.id).await.unwrap();
                info!("{transaction:#?}");
            }
        }

        #[tokio::test]
        #[traced_test]
        async fn test_list_available_addons() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            if let Some(server) = robot.list_servers().await.unwrap().first() {
                for addon in robot.list_available_addons(server.id).await.unwrap() {
                    info!("{addon:#?}");
                }
            }
        }
    }
}
