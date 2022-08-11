use serde::{Deserialize, Serialize};

use crate::{Format, Persist};
use std::{fs::File, path::Path};

pub trait PersistExt: Persist {
    fn save_to_file<K>(&self, path: &dyn AsRef<Path>) -> Result<(), K::SerializeErr>
    where
        K: Format,
        Self: Serialize,
    {
        let path = K::with_ext(path.as_ref());
        let mut file = File::create(path).map_err(serde::ser::Error::custom)?;
        self.save::<K>(&mut file)
    }

    fn load_from_file<K>(path: &dyn AsRef<Path>) -> Result<Self, K::DeserializeErr>
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
