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
        ByteSize::from_str(traffic)
            .map_err(D::Error::custom)
            .map(Some)
    }
}

pub(crate) mod mb {
    use bytesize::{ByteSize, MB};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ByteSize, D::Error> {
        u64::deserialize(deserializer).map(ByteSize::mb)
    }

    pub fn serialize<S>(bytesize: &ByteSize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(bytesize.as_u64() / MB)
    }
}

pub(crate) mod gb {
    use bytesize::{ByteSize, GB};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ByteSize, D::Error> {
        u64::deserialize(deserializer).map(ByteSize::gb)
    }

    pub fn serialize<S>(bytesize: &ByteSize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(bytesize.as_u64() / GB)
    }
}
