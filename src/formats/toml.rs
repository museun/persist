//! Provides a [`Format`] implementation for [`toml`]
//!

use crate::Format;

/// Normal formatted TOML
pub struct Toml;

impl Format for Toml {
    const EXTENSION: &'static str = "toml";
    type SerializeErr = toml::ser::Error;
    type DeserializeErr = toml::de::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        toml::from_slice(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        toml::to_string(data).map(Into::into)
    }
}

/// Pretty formatted TOML
pub struct TomlPretty;

impl Format for TomlPretty {
    const EXTENSION: &'static str = "toml";
    type SerializeErr = toml::ser::Error;
    type DeserializeErr = toml::de::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        toml::from_slice(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        toml::to_string_pretty(data).map(Into::into)
    }
}
