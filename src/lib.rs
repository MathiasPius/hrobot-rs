//! `hrobot` is an unofficial synchronous Rust client for interacting with the [Hetzner Robot API](https://robot.your-server.de/doc/webservice/en.html)
//!
//! See the trait implementations for [`Robot`] for a complete list of supported API Endpoints.
//!
//! **Disclaimer:** the authors are not associated with Hetzner (except as customers), and the crate is in no way endorsed or supported by Hetzner Online GmbH.
//!
//! # Requirements for usage
//! A Hetzner WebService/app user is required to make use of this library.
//!
//! If you already have a Hetzner account, you can create one through the [Hetzner Robot](https://robot.your-server.de) web interface under [Settings/Preferences](https://robot.your-server.de/preferences/index).
//!
//! # Example
//! Here's a quick example showing how to instantiate the [`Robot`] client object
//! and fetching a list of all dedicated servers owned by the account identified by `username`
//! ```no_run
//! use hrobot::*;
//!
//! let client = Robot::new(
//!     &std::env::var("HROBOT_USERNAME").unwrap(),
//!     &std::env::var("HROBOT_PASSWORD").unwrap()
//! );
//!
//! for server in client.list_servers().unwrap() {
//!     println!("{name}: {product} in {location}",
//!         name = server.name,
//!         product = server.product,
//!         location = server.dc
//!     );
//! }
//! ```
//!
//! Running the above example should yield something similar to the anonymized output below
//! ```text
//! foobar: AX51-NVMe in FSN1-DC18
//! ```
pub mod boot;
pub mod error;
pub mod firewall;
pub mod ip;
pub mod keys;
pub mod rdns;
pub mod reset;
pub mod robot;
pub mod server;
pub mod subnet;
pub mod vswitch;
pub mod wol;

pub use boot::*;
pub use error::*;
pub use firewall::*;
pub use ip::*;
pub use keys::*;
pub use rdns::*;
pub use reset::*;
pub use robot::*;
pub use server::*;
pub use subnet::*;
pub use vswitch::*;
pub use wol::*;
