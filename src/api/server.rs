use crate::api::wrapper::{List, Single};
use crate::data::{Cancellation, Server};
use hyper::Uri;
use serde::Serialize;

use super::UnauthenticatedRequest;

pub(crate) fn list_servers() -> UnauthenticatedRequest<List<Server>> {
    UnauthenticatedRequest::new(Uri::from_static("https://robot-ws.your-server.de/server"))
}

pub(crate) fn get_server(server_number: u32) -> UnauthenticatedRequest<Single<Server>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}"
    ))
}

pub(crate) fn rename_server(
    server_number: u32,
    name: &str,
) -> Result<UnauthenticatedRequest<Single<Server>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct RenameServerRequest<'a> {
        pub server_name: &'a str,
    }

    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}"
    ))
    .with_method("POST")
    .with_body(RenameServerRequest { server_name: name })
}

pub(crate) fn get_server_cancellation(
    server_number: u32,
) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
}

pub(crate) fn withdraw_server_cancellation(
    server_number: u32,
) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
    .with_method("DELETE")
}

pub(crate) fn withdraw_server_order(
    server_number: u32,
    reason: Option<&str>,
) -> Result<UnauthenticatedRequest<Single<Cancellation>>, serde_html_form::ser::Error> {
    #[derive(Serialize)]
    struct WithdrawalRequest<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        reversal_reason: Option<&'a str>,
    }

    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/reversal"
    ))
    .with_method("POST")
    .with_body(WithdrawalRequest {
        reversal_reason: reason,
    })
}

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::error::{ApiError, Error};

    #[tokio::test]
    #[traced_test]
    async fn test_list_servers() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_server() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let retrieved_server = robot.get_server(server.id).await.unwrap();

            assert_eq!(retrieved_server.name, server.name);
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_nonexistent_server() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let result = robot.get_server(1).await;
        info!("{result:#?}");

        assert!(matches!(
            result,
            Err(Error::Api(ApiError::ServerNotFound { .. }))
        ));
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "unexpected failure might leave server in renamed state."]
    async fn test_rename_server() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let old_name = &server.name;
            let new_name = "test-rename";

            let renamed_server = robot.rename_server(server.id, &new_name).await.unwrap();
            assert_eq!(renamed_server.name, new_name);

            let rolled_back_server = robot.rename_server(server.id, &old_name).await.unwrap();
            assert_eq!(&rolled_back_server.name, old_name);
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_server_cancellation() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let status = robot.get_server_cancellation(server.id).await.unwrap();
            info!("{status:#?}");
            assert!(!status.cancelled);
        }
    }
}
