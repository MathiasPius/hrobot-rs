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

pub(crate) mod mib {
    use bytesize::{ByteSize, MIB};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ByteSize, D::Error> {
        u64::deserialize(deserializer).map(ByteSize::mib)
    }

    pub fn serialize<S>(bytesize: &ByteSize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(bytesize.as_u64() / MIB)
    }
}

pub(crate) mod gib {
    use bytesize::{ByteSize, GIB};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ByteSize, D::Error> {
        u64::deserialize(deserializer).map(ByteSize::gib)
    }

    pub fn serialize<S>(bytesize: &ByteSize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(bytesize.as_u64() / GIB)
    }
}
