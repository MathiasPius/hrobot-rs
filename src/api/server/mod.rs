//! Server structs and implementations.

mod models;

use crate::{
    api::wrapper::{List, Single},
    error::Error,
    AsyncRobot,
};
use hyper::Uri;
use serde::Serialize;

pub use models::*;

use super::{wrapper::Empty, UnauthenticatedRequest};

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
    cancellation: Cancel,
) -> Result<UnauthenticatedRequest<Single<Cancelled>>, serde_html_form::ser::Error> {
    let cancellation: InternalCancel = cancellation.into();
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
    .with_method("POST")
    .with_body(cancellation)
}

fn withdraw_server_cancellation(server_number: ServerId) -> UnauthenticatedRequest<Empty> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/server/{server_number}/cancellation"
    ))
    .with_method("DELETE")
}

impl AsyncRobot {
    /// List all owned servers.
    ///
    /// # Example
    /// Print the ids and names of all servers accessible by our credentials.
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let _ = dotenvy::dotenv().ok();
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
    /// # use hrobot::api::server::{ServerId, Cancel};
    /// # use time::{Date, Month};
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.cancel_server(ServerId(1234567), Cancel {
    ///     date: Some(Date::from_calendar_date(2023, Month::June, 10).unwrap()),
    ///     reason: Some("Server no longer necessary due to project ending".to_string()),
    ///     reserved: false
    /// }).await.unwrap();
    /// # }
    /// ```
    pub async fn cancel_server(
        &self,
        server_number: ServerId,
        cancellation: Cancel,
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
    /// # use hrobot::api::server::ServerId;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.withdraw_server_cancellation(ServerId(1234567)).await.unwrap();
    /// # }
    /// ```
    pub async fn withdraw_server_cancellation(&self, server_number: ServerId) -> Result<(), Error> {
        self.go(withdraw_server_cancellation(server_number))
            .await?
            .throw_away();

        Ok(())
    }
}
