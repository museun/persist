pub struct Toml;

impl crate::Format for Toml {
    const EXTENSION: &'static str = "toml";
    type SerializeErr = toml::ser::Error;
    type DeserializeErr = toml::de::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        for<'de> T: serde::Deserialize<'de>,
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

pub struct TomlPretty;

impl crate::Format for TomlPretty {
    const EXTENSION: &'static str = "toml";
    type SerializeErr = toml::ser::Error;
    type DeserializeErr = toml::de::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        for<'de> T: serde::Deserialize<'de>,
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
