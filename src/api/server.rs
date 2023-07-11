use crate::api::wrapper::{deserialize_inner, deserialize_inner_vec};
use crate::data::Server;
use hyper::Uri;
use serde::{Deserialize, Serialize};

use super::UnauthenticatedRequest;

pub fn list_servers() -> UnauthenticatedRequest<ListServerResponse> {
    UnauthenticatedRequest::new(Uri::from_static("https://robot-ws.your-server.de/server"))
}

pub fn get_server(server_number: u32) -> UnauthenticatedRequest<GetServerResponse> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}"
    ))
}

pub fn rename_server(
    server_number: u32,
    name: &str,
) -> Result<UnauthenticatedRequest<RenameServerResponse>, serde_html_form::ser::Error> {
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

#[derive(Debug, Deserialize)]
pub struct ListServerResponse(#[serde(deserialize_with = "deserialize_inner_vec")] pub Vec<Server>);

#[derive(Debug, Deserialize)]
pub struct GetServerResponse(#[serde(deserialize_with = "deserialize_inner")] pub Server);

#[derive(Debug, Deserialize)]
pub struct RenameServerResponse(#[serde(deserialize_with = "deserialize_inner")] pub Server);

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use tracing::{info, Level};

    #[tokio::test]
    async fn test_list_servers() {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::fmt()
            .with_max_level(Level::TRACE)
            .pretty()
            .init();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");
    }

    #[tokio::test]
    async fn test_get_server() {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::fmt()
            .with_max_level(Level::TRACE)
            .pretty()
            .init();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.iter().next() {
            let retrieved_server = robot.get_server(server.id).await.unwrap();

            assert_eq!(retrieved_server.name, server.name);
        }
    }

    #[tokio::test]
    #[ignore = "unexpected failure might leave server in renamed state."]
    async fn test_rename_server() {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt::fmt()
            .with_max_level(Level::TRACE)
            .pretty()
            .init();

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
}
