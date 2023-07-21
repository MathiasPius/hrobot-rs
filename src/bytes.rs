use std::str::FromStr;

pub use bytesize::ByteSize;
use serde::{de::Error, Deserialize, Deserializer};

pub(crate) fn traffic<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<ByteSize>, D::Error> {
    let traffic = <&str>::deserialize(deserializer)?;
    
    if traffic == "unlimited" {
        Ok(None)
    } else {
        ByteSize::from_str(traffic).map_err(D::Error::custom).map(Some)
    }
}