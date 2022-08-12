use serde::{Deserialize, Serialize};

use crate::{Format, Persist};
use std::{fs::File, path::Path};

/// Extension trait to provide useful helpers for the [`Persist`] trait
pub trait PersistExt: Persist {
    /// Save this type, with the provided [`Format`] to the [`Path`] provided
    fn save_to_file<K>(&self, path: impl AsRef<Path>) -> Result<(), K::SerializeErr>
    where
        K: Format,
        Self: Serialize,
    {
        let path = K::with_ext(path.as_ref());
        let mut file = File::create(path).map_err(serde::ser::Error::custom)?;
        self.save::<K>(&mut file)
    }

    /// Load this type, with the provided [`Format`] from the [`Path`] provided
    fn load_from_file<K>(path: impl AsRef<Path>) -> Result<Self, K::DeserializeErr>
    where
        K: Format,
        Self: for<'de> Deserialize<'de>,
    {
        let path = K::with_ext(path.as_ref());
        let mut file = File::open(path).map_err(serde::de::Error::custom)?;
        Self::load::<K>(&mut file)
    }
}

impl<T> PersistExt for T where T: Persist {}
