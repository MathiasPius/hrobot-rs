//! `hrobot` is an unofficial asynchronous Rust client for interacting with the [Hetzner Robot API](https://robot.your-server.de/doc/webservice/en.html)
//!
//! See the [`AsyncRobot`] struct for a complete list of supported API Endpoints.
//!
//! **Disclaimer:** the authors are not associated with Hetzner (except as customers), and the crate is in no way endorsed or supported by Hetzner Online GmbH.
//!
//! # Requirements for usage
//! A Hetzner WebService/app user is required to make use of this library.
//!
//! If you already have a Hetzner account, you can create one through the [Hetzner Robot](https://robot.your-server.de) web interface under [Settings/Preferences](https://robot.your-server.de/preferences/index).
//!
//! # Example
//! Here's a quick example showing how to instantiate the [`AsyncRobot`] client object
//! and fetching a list of all dedicated servers owned by the account identified by `username`
//! ```rust,no_run
//! use hrobot::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Robot is instantiated using the environment
//!     // variables HROBOT_USERNAME an HROBOT_PASSWORD.
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
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![forbid(unsafe_code)]
pub mod api;
pub mod error;

mod client;
mod conversion;
mod urlencode;

pub use ::bytesize;
pub use ::rust_decimal;
pub use ::time;
pub use client::*;