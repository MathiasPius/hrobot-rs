use crate::models::Firewall;

use super::{wrapper::Single, UnauthenticatedRequest};

pub(crate) fn get_firewall(server_number: u32) -> UnauthenticatedRequest<Single<Firewall>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/firewall/{server_number}"
    ))
}

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_get_firewall() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let firewall = robot.get_firewall(server.id).await.unwrap();

            info!("{firewall:#?}");
        }
    }
}
