//! `hrobot` is an unofficial asynchronous Rust client for interacting with the [Hetzner Robot API](https://robot.your-server.de/doc/webservice/en.html)
//!
//! See the [`AsyncRobot`](crate::AsyncRobot) struct for a complete list of supported API Endpoints.
//!
//! **Disclaimer:** the authors are not associated with Hetzner (except as customers), and the crate is in no way endorsed or supported by Hetzner Online GmbH.
//!
//! # Requirements for usage
//! A Hetzner WebService/app user is required to make use of this library.
//!
//! If you already have a Hetzner account, you can create one through the [Hetzner Robot](https://robot.your-server.de) web interface under [Settings/Preferences](https://robot.your-server.de/preferences/index).
//!
//! # Example
//! Here's a quick example showing how to instantiate the [`AsyncRobot`](crate::AsyncRobot) client object
//! and fetching a list of all dedicated servers owned by the account identified by `username`
//! ```rust,no_run
//! use hrobot::*;
//!
//! // Robot is instantiated using the environment
//! // variables HROBOT_USERNAME an HROBOT_PASSWORD.
//! #[tokio::main]
//! async fn main() {
//!     let robot = AsyncRobot::default();
//!
//!     for server in robot.list_servers().await.unwrap() {
//!         println!("{name}: {product} in {location}",
//!             name = server.name,
//!             product = server.product,
//!             location = server.dc
//!         );
//!     }
//! }
//! ```
//!
//! Running the above example should yield something similar to the output below:
//! ```text
//! foo: AX51-NVMe in FSN1-DC18
//! bar: Server Auction in FSN1-DC5
//! ```
pub mod api;
pub mod error;

mod client;
mod timezones;
mod urlencode;
pub mod bytes;

pub use ::time;
pub use client::*;
