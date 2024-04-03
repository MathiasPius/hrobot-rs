use hrobot::{
    api::server::{Server, ServerId},
    AsyncRobot,
};

#[allow(unused)]
pub fn provisioned_server_id() -> ServerId {
    dotenvy::dotenv().ok();

    ServerId(
        u32::from_str_radix(
            std::env::var("HETZNER_INTEGRATION_TEST_SERVER_ID")
                .as_deref()
                .unwrap(),
            10,
        )
        .unwrap(),
    )
}

#[allow(unused)]
pub async fn provisioned_server() -> Server {
    let id = provisioned_server_id();
    let robot = AsyncRobot::default();

    robot.get_server(id).await.unwrap()
}
