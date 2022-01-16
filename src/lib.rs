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

/// Utility function for coercing a string or list of strings into a Vec<String> during deserialization
/// Source: https://stackoverflow.com/questions/41151080/deserialize-a-json-string-or-array-of-strings-into-a-vec
pub(crate) fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StringOrVec(std::marker::PhantomData<Vec<String>>);

    impl<'de> serde::de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(std::marker::PhantomData))
}

pub(crate) fn num_or_seq_num<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct NumOrVec(std::marker::PhantomData<Vec<String>>);

    impl<'de> serde::de::Visitor<'de> for NumOrVec {
        type Value = Vec<u64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            serde::Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(NumOrVec(std::marker::PhantomData))
}
