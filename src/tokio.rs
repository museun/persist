use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, path::Path, pin::Pin};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};

use crate::Format;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait Persist
where
    Self: Send + Sync,
{
    fn save<'a, K>(
        &'a self,
        out: &'a mut (dyn AsyncWrite + Unpin + Send),
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

    fn load<K>(
        input: &mut (dyn AsyncRead + Unpin + Send),
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

pub trait PersistExt
where
    Self: Persist,
{
    fn save_to_file<'a, K>(
        &'a self,
        path: &'a (dyn AsRef<Path> + Send + Sync),
    ) -> BoxFuture<'a, Result<(), K::SerializeErr>>
    where
        Self: Serialize,
        K: Format + Send,
        K::SerializeErr: Send,
    {
        Box::pin(async {
            let path = K::with_ext(path.as_ref());
            let mut file = File::create(path)
                .await
                .map_err(serde::ser::Error::custom)?;
            self.save::<K>(&mut file).await
        })
    }

    fn load_from_file<K>(
        path: &(dyn AsRef<Path> + Send + Sync),
    ) -> BoxFuture<'_, Result<Self, K::DeserializeErr>>
    where
        Self: DeserializeOwned,
        K: Format + Send,
        K::DeserializeErr: Send,
    {
        Box::pin(async {
            let path = K::with_ext(path.as_ref());
            let mut file = File::open(path).await.map_err(serde::de::Error::custom)?;
            Self::load::<K>(&mut file).await
        })
    }
}

impl<T> PersistExt for T where T: Persist {}
