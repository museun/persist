use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub trait Format {
    const EXTENSION: &'static str;

    type SerializeErr: serde::ser::Error;
    type DeserializeErr: serde::de::Error;

    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        for<'de> T: Deserialize<'de>;

    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: Serialize + ?Sized;

    fn with_ext(path: &Path) -> PathBuf {
        path.with_extension(Self::EXTENSION)
    }
}
