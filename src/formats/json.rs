//! Provides a [`Format`] implementation for [`serde_json`]
//!

use crate::Format;

/// Normal formatted JSON
pub struct Json;

impl Format for Json {
    const EXTENSION: &'static str = "json";
    type SerializeErr = serde_json::Error;
    type DeserializeErr = serde_json::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_slice(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        serde_json::to_vec(data)
    }
}

/// Pretty formatted JSON
pub struct JsonPretty;

impl Format for JsonPretty {
    const EXTENSION: &'static str = "json";
    type SerializeErr = serde_json::Error;
    type DeserializeErr = serde_json::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_json::from_slice(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        serde_json::to_vec_pretty(data)
    }
}
