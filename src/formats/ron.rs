//! Provides a [`Format`] implementation for [`ron`]
//!

use crate::Format;

/// Normal formatted RON
pub struct Ron;

impl Format for Ron {
    const EXTENSION: &'static str = "ron";
    type SerializeErr = ron::Error;
    type DeserializeErr = ron::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        ron::de::from_bytes(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        ron::to_string(data).map(Into::into)
    }
}
