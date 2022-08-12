use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A trait to a provide **format** implemention.
///
/// This is used in the [`crate::Persist`] trait to determine how to serialize/deserialize the type
pub trait Format {
    /// The extension used for this format
    const EXTENSION: &'static str;

    /// The error returned by serialization
    type SerializeErr: serde::ser::Error;

    /// The error returned by deserialization
    type DeserializeErr: serde::de::Error;

    /// Deserialize a type from a byte slice
    fn from_slice<T>(data: &[u8]) -> Result<T, Self::DeserializeErr>
    where
        T: for<'de> Deserialize<'de>;

    /// Serialize a type to a byte vector
    fn to_vec<T>(data: &T) -> Result<Vec<u8>, Self::SerializeErr>
    where
        T: Serialize + ?Sized;

    /// Combines the provided [`Path`] with the pre-determined [`Self::EXTENSION`]
    ///
    /// This'll replace any extension in the path with the new one
    ///
    /// Or add the extension if its missing
    fn with_ext(path: &Path) -> PathBuf {
        path.with_extension(Self::EXTENSION)
    }
}
