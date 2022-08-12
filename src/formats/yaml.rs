//! Provides a [`Format`] implementation for [`serde_yaml`]
//!

/// Normal formatted YAML
///
/// **NOTE** the extension is `yaml`, rather than `yml`
pub struct Yaml;

use crate::Format;

impl Format for Yaml {
    const EXTENSION: &'static str = "yaml";
    type SerializeErr = serde_yaml::Error;
    type DeserializeErr = serde_yaml::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serde_yaml::from_slice(data)
    }

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: serde::Serialize + ?Sized,
    {
        serde_yaml::to_string(data).map(Into::into)
    }
}
