mod models;

use crate::{
    api::wrapper::{List, Single},
    error::Error,
    AsyncHttpClient, AsyncRobot,
};
use hyper::Uri;
use serde::Serialize;

pub use models::*;

use super::UnauthenticatedRequest;

fn list_servers() -> UnauthenticatedRequest<List<Server>> {
    UnauthenticatedRequest::new(Uri::from_static("https://robot-ws.your-server.de/server"))
}

fn get_server(server_number: ServerId) -> UnauthenticatedRequest<Single<Server>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}"
    ))
}

fn rename_server(
    server_number: ServerId,
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

fn get_server_cancellation(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
}

fn cancel_server(
    server_number: ServerId,
    cancellation: Cancelled,
) -> Result<UnauthenticatedRequest<Single<Cancelled>>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
    .with_method("POST")
    .with_body(cancellation)
}

fn withdraw_server_cancellation(
    server_number: ServerId,
) -> UnauthenticatedRequest<Single<Cancellation>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
    .with_method("DELETE")
}

fn withdraw_server_order(
    server_number: ServerId,
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

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// List all owned servers.
    ///
    /// # Example
    /// Print the ids and names of all servers accessible by our credentials.
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() {
    /// # dotenvy::dotenv().ok();
    /// let robot = hrobot::AsyncRobot::default();
    /// for server in robot.list_servers().await.unwrap() {
    ///     println!("{}: {}", server.id, server.name);
    /// }
    /// # }
    /// ```
    pub async fn list_servers(&self) -> Result<Vec<Server>, Error> {
        Ok(self.go(list_servers()).await?.0)
    }

    /// Retrieve complete information about a specific [`Server`].
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let server = robot.get_server(ServerId(1234567)).await.unwrap();
    /// assert_eq!(server.id, 1234567);
    /// println!("Name: {}", server.name);
    /// # }
    /// ```
    pub async fn get_server(&self, server_number: ServerId) -> Result<Server, Error> {
        Ok(self.go(get_server(server_number)).await?.0)
    }

    /// Rename a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.rename_server(ServerId(1234567), "gibson").await.unwrap();
    /// # }
    /// ```
    pub async fn rename_server(
        &self,
        server_number: ServerId,
        name: &str,
    ) -> Result<Server, Error> {
        Ok(self.go(rename_server(server_number, name)?).await?.0)
    }

    /// Get the current cancellation status of a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::ServerId;
    /// # use hrobot::api::server::Cancellation;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let cancellation = robot.get_server_cancellation(ServerId(1234567)).await.unwrap();
    /// assert!(matches!(
    ///     cancellation,
    ///     Cancellation::Cancellable(_)
    /// ));
    /// # }
    /// ```
    pub async fn get_server_cancellation(
        &self,
        server_number: ServerId,
    ) -> Result<Cancellation, Error> {
        Ok(self.go(get_server_cancellation(server_number)).await?.0)
    }

    /// Get the current cancellation status of a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::{ServerId, Cancelled};
    /// # use time::{Date, Month};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_server(ServerId(1234567), Cancelled {
    ///     date: Date::from_calendar_date(2023, Month::June, 10).unwrap(),
    ///     reason: Some("Server no longer necessary due to project ending".to_string()),
    ///     reserved: false
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_server(
        &self,
        server_number: ServerId,
        cancellation: Cancelled,
    ) -> Result<Cancelled, Error> {
        Ok(self
            .go(cancel_server(server_number, cancellation)?)
            .await?
            .0)
    }

    /// Withdraw a server cancellation.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::{ServerId, Cancellation};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let cancellation = robot.withdraw_server_cancellation(ServerId(1234567)).await.unwrap();
    /// assert!(matches!(
    ///     cancellation,
    ///     Cancellation::Cancellable(_)
    /// ));
    /// # }
    /// ```
    pub async fn withdraw_server_cancellation(
        &self,
        server_number: ServerId,
    ) -> Result<Cancellation, Error> {
        Ok(self
            .go(withdraw_server_cancellation(server_number))
            .await?
            .0)
    }

    /// Withdraw a server order.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::server::{ServerId, Cancellation};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// let cancellation = robot.withdraw_server_order(ServerId(1234567), Some("Accidental purchase.")).await.unwrap();
    /// assert!(matches!(
    ///     cancellation,
    ///     Cancellation::Cancelled(_)
    /// ));
    /// # }
    /// ```
    pub async fn withdraw_server_order(
        &self,
        server_number: ServerId,
        reason: Option<&str>,
    ) -> Result<Cancellation, Error> {
        Ok(self
            .go(withdraw_server_order(server_number, reason)?)
            .await?
            .0)
    }
}

#[cfg(all(test, feature = "hyper-client"))]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;

    use crate::{
        api::server::{Cancellation, ServerId},
        error::{ApiError, Error},
    };

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

        if let Some(server) = servers.first() {
            let retrieved_server = robot.get_server(server.id).await.unwrap();

            assert_eq!(retrieved_server.name, server.name);
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn test_get_nonexistent_server() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let result = robot.get_server(ServerId(1)).await;
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

        if let Some(server) = servers.first() {
            let old_name = &server.name;
            let new_name = "test-rename";

            let renamed_server = robot.rename_server(server.id, new_name).await.unwrap();
            assert_eq!(renamed_server.name, new_name);

            let rolled_back_server = robot.rename_server(server.id, old_name).await.unwrap();
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

        if let Some(server) = servers.first() {
            let status = robot.get_server_cancellation(server.id).await.unwrap();
            info!("{status:#?}");
            assert!(matches!(status, Cancellation::Cancellable(_)));
        }
    }
}
