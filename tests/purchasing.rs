mod common;

use hrobot::{
    api::ordering::{
        AddonId, AddonOrder, AuthorizationMethod, ImSeriousAboutSpendingMoney, MarketProductOrder,
        ProductOrder,
    },
    error::{ApiError, Error},
    AsyncRobot,
};
use rust_decimal::Decimal;
use tracing::info;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn get_product_listing() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    for product in robot.list_products(.., .., None).await.unwrap() {
        info!("{product:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn get_single_product() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    if let Some(product) = robot.list_products(.., .., None).await.unwrap().first() {
        let product = robot.get_product(&product.id).await.unwrap();
        info!("{product:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn list_recent_product_transactions() {
    let _ = dotenvy::dotenv().ok();

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
async fn get_recent_product_transactions() {
    let _ = dotenvy::dotenv().ok();

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
async fn list_market_products() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    for product in robot.list_market_products().await.unwrap() {
        info!("{product:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn get_single_market_product() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    if let Some(product) = robot.list_market_products().await.unwrap().first() {
        let product = robot.get_market_product(&product.id).await.unwrap();
        info!("{product:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn list_recent_market_transactions() {
    let _ = dotenvy::dotenv().ok();

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
async fn get_recent_market_transactions() {
    let _ = dotenvy::dotenv().ok();

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
async fn list_available_addons() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    if let Some(server) = robot.list_servers().await.unwrap().first() {
        for addon in robot.list_available_addons(server.id).await.unwrap() {
            info!("{addon:#?}");
        }
    }
}

#[tokio::test]
#[traced_test]
async fn dry_run_purchase_cheap_server() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let fingerprint = robot
        .list_ssh_keys()
        .await
        .unwrap()
        .pop()
        .unwrap()
        .fingerprint;

    let products = robot.list_products(.., ..=0, None).await.unwrap();
    println!("{products:#?}");

    let Some((product, location, price)) = products.into_iter().find_map(|product| {
        for (location, price) in product.prices {
            if price.setup.net.is_zero() {
                return Some((product.id, location, price.setup.net));
            }
        }

        None
    }) else {
        panic!("no products with free setup available.")
    };

    println!("ordering {product} in {location} for €{price}");
    assert_eq!(price, Decimal::ZERO);

    let order = ProductOrder {
        id: product,
        auth: AuthorizationMethod::Keys(vec![fingerprint]),
        distribution: Some("Rescue system".to_string()),
        language: Some("en".to_string()),
        location,
        addons: vec![AddonId::from("primary_ipv4")],
        comment: None,
        i_want_to_spend_money_to_purchase_a_server: ImSeriousAboutSpendingMoney::NoThisIsJustATest,
    };

    let result = robot.place_product_order(order).await.unwrap();

    info!("{result:#?}");
}

#[tokio::test]
#[traced_test]
async fn dry_run_purchase_auction_server() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let fingerprint = robot
        .list_ssh_keys()
        .await
        .unwrap()
        .pop()
        .unwrap()
        .fingerprint;

    let mut products = robot.list_market_products().await.unwrap();
    products.sort_by_key(|product| product.price.recurring.net);

    if let Some(cheapest) = products.first() {
        let order = MarketProductOrder {
            id: cheapest.id,
            auth: AuthorizationMethod::Keys(vec![fingerprint]),
            distribution: Some("Rescue system".to_string()),
            language: Some("en".to_string()),
            addons: vec![AddonId::from("primary_ipv4")],
            comment: None,
            i_want_to_spend_money_to_purchase_a_server:
                ImSeriousAboutSpendingMoney::NoThisIsJustATest,
        };

        let result = robot.place_market_order(order).await.unwrap();

        info!("{result:#?}");
    }
}

#[tokio::test]
#[traced_test]
async fn purchase_additional_ipv4() {
    let _ = dotenvy::dotenv().ok();

    let robot = AsyncRobot::default();

    let server_id = common::provisioned_server_id();

    let transaction = robot
        .place_addon_order(AddonOrder {
            id: AddonId::from("additional_ipv4"),
            server: server_id,
            reason: Some("VPS".to_string()),
            gateway: None,
            i_want_to_spend_money_to_purchase_an_addon:
                ImSeriousAboutSpendingMoney::NoThisIsJustATest,
        })
        .await
        .unwrap();

    info!("{transaction:#?}");
}
