//! This is an _async_ implementation of [`Persist`] and [`PersistExt`] using [`tokio`]
//!

use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, path::Path, pin::Pin};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

use crate::Format;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The base [`Persist`] trait, this provides both [`Persist::save`] and [`Persist::load`] for the provided [`Format`]
///
/// This gets implemented for every type that implements [`serde::Serialize`] and [`serde::Deserialize`]
pub trait Persist
where
    Self: Send + Sync,
{
    /// Serialize this type with the provided [`Format`] to the writer
    fn save<'a, K>(
        &'a self,
        out: &'a mut (impl AsyncWrite + Unpin + Send),
    ) -> BoxFuture<'a, Result<(), K::SerializeErr>>
    where
        Self: Serialize,
        K: Format + Send,
        K::SerializeErr: Send,
    {
        Box::pin(async {
            let data = K::to_vec(self)?;
            out.write_all(&data)
                .await
                .map_err(serde::ser::Error::custom)
        })
    }

    /// Serialize this type with the provided [`Format`] from the reader
    fn load<K>(
        input: &mut (impl AsyncRead + Unpin + Send),
    ) -> BoxFuture<'_, Result<Self, K::DeserializeErr>>
    where
        Self: DeserializeOwned,
        K: Format + Send,
        K::DeserializeErr: Send,
    {
        Box::pin(async {
            let mut out = vec![];
            input
                .read_to_end(&mut out)
                .await
                .map_err(serde::de::Error::custom)?;
            K::from_slice(&out)
        })
    }
}

impl<T> Persist for T where T: DeserializeOwned + Serialize + Send + Sync {}

/// Extension trait to provide useful helpers for the [`Persist`] trait
pub trait PersistExt
where
    Self: Persist,
{
    /// Save this type, with the provided [`Format`] to the [`Path`] provided
    fn save_to_file<K>(&self, path: impl AsRef<Path>) -> BoxFuture<'_, Result<(), K::SerializeErr>>
    where
        Self: Serialize,
        K: Format + Send,
        K::SerializeErr: Send,
    {
        let path = K::with_ext(path.as_ref());
        Box::pin(async {
            let mut file = File::create(path)
                .await
                .map_err(serde::ser::Error::custom)?;
            self.save::<K>(&mut file).await
        })
    }

    /// Load this type, with the provided [`Format`] from the [`Path`] provided
    fn load_from_file<'a, K>(
        path: impl AsRef<Path>,
    ) -> BoxFuture<'a, Result<Self, K::DeserializeErr>>
    where
        Self: DeserializeOwned,
        K: Format + Send,
        K::DeserializeErr: Send,
    {
        let path = K::with_ext(path.as_ref());
        Box::pin(async {
            let mut file = File::open(path).await.map_err(serde::de::Error::custom)?;
            Self::load::<K>(&mut file).await
        })
    }
}

impl<T> PersistExt for T where T: Persist {}
