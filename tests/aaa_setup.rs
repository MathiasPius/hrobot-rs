use hrobot::{
    api::{
        ordering::{AddonId, AuthorizationMethod, ImSeriousAboutSpendingMoney, MarketProductOrder},
        server::ServerId,
    },
    AsyncRobot,
};
use serial_test::file_serial;
use tracing::info;
use tracing_test::traced_test;

mod common;

#[tokio::test]
#[traced_test]
#[file_serial]
async fn provision_cheapest_server() {
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

    let cheapest = products.first().unwrap();

    let order = MarketProductOrder {
        id: cheapest.id,
        auth: AuthorizationMethod::Keys(vec![fingerprint.clone()]),
        distribution: Some("Rescue system".to_string()),
        language: Some("en".to_string()),
        addons: vec![AddonId::from("primary_ipv4")],
        comment: None,
        i_want_to_spend_money_to_purchase_a_server:
            ImSeriousAboutSpendingMoney::LetMeSpendMyMoneyAlready,
    };

    let result = robot.place_market_order(order).await.unwrap();

    info!("{result:#?}");

    let id = result
        .server_id
        .unwrap_or_else(|| ServerId(result.product.id.0));

    std::fs::write(common::PROVISIONED_SERVER_ID_PATH, id.to_string()).unwrap();
}
