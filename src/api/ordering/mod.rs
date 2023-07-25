mod models;
use std::ops::RangeInclusive;

pub use models::*;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::{error::Error, AsyncRobot};

use super::{wrapper::List, UnauthenticatedRequest};

fn list_products(
    monthly_price: Option<RangeInclusive<Decimal>>,
    setup_price: Option<RangeInclusive<Decimal>>,
    location: Option<&str>,
) -> Result<UnauthenticatedRequest<List<Product>>, serde_html_form::ser::Error> {
    #[derive(Debug, Serialize)]
    struct ProductSearch<'a> {
        min_price: Option<Decimal>,
        max_price: Option<Decimal>,
        min_price_setup: Option<Decimal>,
        max_price_setup: Option<Decimal>,
        location: Option<&'a str>,
    }

    UnauthenticatedRequest::from("https://robot-ws.your-server.de/order/server/product").with_body(
        ProductSearch {
            min_price: monthly_price.as_ref().map(|p| *p.start()),
            max_price: monthly_price.as_ref().map(|p| *p.end()),
            min_price_setup: setup_price.as_ref().map(|p| *p.start()),
            max_price_setup: setup_price.as_ref().map(|p| *p.end()),
            location,
        },
    )
}

impl AsyncRobot {
    pub async fn list_products(
        &self,
        monthly_price: Option<RangeInclusive<Decimal>>,
        setup_price: Option<RangeInclusive<Decimal>>,
        location: Option<&str>,
    ) -> Result<Vec<Product>, Error> {
        Ok(self
            .go(list_products(monthly_price, setup_price, location)?)
            .await?
            .0)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "non-disruptive-tests")]
    mod non_disruptive_tests {
        use tracing::info;
        use tracing_test::traced_test;

        use crate::AsyncRobot;

        #[tokio::test]
        #[traced_test]
        async fn get_product_listing() {
            dotenvy::dotenv().ok();

            let robot = AsyncRobot::default();

            for product in robot.list_products(None, None, None).await.unwrap() {
                info!("{product:#?}");
            }
        }
    }
}
