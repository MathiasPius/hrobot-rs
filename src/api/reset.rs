use serde::{Deserialize, Serialize};

use crate::{error::Error, AsyncHttpClient, AsyncRobot};

use super::{wrapper::Single, UnauthenticatedRequest};

fn get_reset_options(server_number: u32) -> UnauthenticatedRequest<Single<ResetOptions>> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/reset/{server_number}"
    ))
}

fn trigger_reset(
    server_number: u32,
    reset: Reset,
) -> Result<UnauthenticatedRequest<ExecutedReset>, serde_html_form::ser::Error> {
    UnauthenticatedRequest::from(&format!(
        "https://robot-ws.your-server.de/reset/{server_number}"
    ))
    .with_method("POST")
    .with_body(ExecutedReset { reset })
}

impl<Client: AsyncHttpClient> AsyncRobot<Client> {
    /// Retrieve list of reset options for a server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.get_reset_options(1234567).await.unwrap();
    /// # }
    /// ```
    pub async fn get_reset_options(&self, server_number: u32) -> Result<Vec<Reset>, Error> {
        Ok(self.go(get_reset_options(server_number)).await?.0.options)
    }

    /// Trigger a reset for the server.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use hrobot::api::reset::Reset;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let robot = hrobot::AsyncRobot::default();
    /// robot.trigger_reset(1234567, Reset::Power).await.unwrap();
    /// # }
    /// ```
    pub async fn trigger_reset(&self, server_number: u32, reset: Reset) -> Result<Reset, Error> {
        Ok(self.go(trigger_reset(server_number, reset)?).await?.reset)
    }
}

#[derive(Serialize, Deserialize)]
struct ExecutedReset {
    #[serde(rename = "type")]
    reset: Reset,
}

#[derive(Deserialize)]
struct ResetOptions {
    #[serde(rename = "type")]
    options: Vec<Reset>,
}

/// Kind of reset to perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reset {
    /// Request a manual power cycle, by Hetzner staff.
    ///
    /// The manual power cycle (cold reset) option will generate an email that will
    /// be sent directly to Hetzner's data center. One of Hetzner's technicians will
    /// then disconnect the server from the power supply, reconnect it, and thereby
    /// restart the system. The technician will send you an email once they have
    /// restarted the system and it is reachable again.
    /// If you cannot reach the server after the power cycle, the technician will
    /// connect a remote console to your server and send you the login details.
    ///
    /// The manual power cycle can be reasonable as part of a trouble-shooting process;
    /// however, it is a more drastic option. We advise you to consider the following
    /// aspects before using it:
    ///
    /// * Have you tried other, less drastic reset options? Or have you considered
    ///   ordering a remote console (Support; Product; Remote Console)?
    /// * If your server has an IPMI (e.g. iDRAC with Dell servers), you can use it
    ///   to inspect the screen output of the server and conduct a restart.
    ///
    /// **Warning**: Hetzner's technicians will not inspect the state of the server
    /// before the power cycle. If you would like us to provide you with information
    /// on the state of the system or to process your request in a specific way,
    /// please open a suitable support request in the support section (e.g. Support;
    /// Product; Technical; Server is unstable) and let us know how we can help you.    
    #[serde(rename = "man")]
    Manual,

    /// Trigger a software reset.
    ///
    /// This is equivalent to hitting Ctrl+Alt+Del.
    #[serde(rename = "sw")]
    Software,

    /// Execute an automatic hardware reset
    ///
    /// What happens in the background here is exactly the same as when you press the
    /// reset button on your home PC.
    #[serde(rename = "hw")]
    Hardware,

    /// Press power button of server.
    ///
    /// If the server is powered down, it can be turned back on with this function.
    /// If the server is still running, it will receive an ACPI signal to shut down.
    /// Hetzner's servers and standard images are configured so that this process triggers
    /// a regular operating system shutdown. What happens is actually exactly the same as
    /// what happens when you press the power button on your home computer.
    #[serde(rename = "power")]
    Power,

    /// Long press of the server's power button.
    ///
    /// This option forces the server to immediately shut off. It should only be used in
    /// cases where the system is unresponsive to a graceful shut-down.
    #[serde(rename = "power_long")]
    PowerLong,

    /// Undocumented reset method.
    #[serde(untagged)]
    Other(String),
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use tracing::info;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    #[serial("boot-configuration")]
    async fn test_list_reset_options() {
        dotenvy::dotenv().ok();

        let robot = crate::AsyncRobot::default();

        let servers = robot.list_servers().await.unwrap();
        info!("{servers:#?}");

        if let Some(server) = servers.first() {
            let reset_options = robot.get_reset_options(server.id).await.unwrap();

            info!("{reset_options:#?}");
        }
    }
}
