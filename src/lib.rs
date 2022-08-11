use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub trait Persist {
    fn save<K>(&self, output: &mut dyn Write) -> Result<(), K::SerializeErr>
    where
        K: Format,
        Self: Serialize,
    {
        let data = K::to_vec(self)?;
        output.write_all(&data).map_err(serde::ser::Error::custom)
    }

    fn load<K>(input: &mut dyn Read) -> Result<Self, K::DeserializeErr>
    where
        K: Format,
        Self: for<'de> Deserialize<'de>,
    {
        let mut out = vec![];
        input
            .read_to_end(&mut out)
            .map_err(serde::de::Error::custom)?;
        K::from_slice(&out)
    }
}

impl<T> Persist for T where for<'de> T: Deserialize<'de> + Serialize {}

mod format;
pub use format::Format;

mod ext;
pub use ext::PersistExt;

#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(feature = "toml")]
pub mod toml;
