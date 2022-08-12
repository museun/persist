//! # What this crate is
//! This crate provides an easy way to save and load a type to a file.
//!
//! - allows you to switch out the 'format provider'.
//! - allows you to change your mind on how things should be represented on disk.
//! - also allows you to transcode between formats
//!
//! # Table of contents
//! - [Feature flags](#feature-flags)
//! - [Providing a format](#implementing-your-own-format)
//! - [Example](#example-of-using-this-crate)
//!
//! # Feature flags
//! By default, only the synchronous traits are provided
//!
//! Below are tables of optional features you can enable:
//! * [Formats](#provided-formats)
//! * [Runtime](#runtime-support)
//! #### Provided formats
//! |name|provides|
//! |---|---|
//! |`json`|[`Format`] implementation for [`serde_json`](::serde_json)|
//! |`yaml`|[`Format`] implementation for [`serde_yaml`](::serde_yaml)|
//! |`toml`|[`Format`] implementation for [`toml`](::toml)|
//! |`ron`|[`Format`] implementation for [`ron`](::ron)|
//! #### Runtime support
//! |name|provides|
//! |---|---|
//! |[`tokio`](https://docs.rs/tokio/1.20.1/tokio/)|asynchronous [`Persist`](crate::tokio::Persist) and [`PersistExt`](crate::tokio::PersistExt)
//!
//! # Implementing your own [`Format`]
//!
//! To implement your own format, just implement the [`Format`] trait on a unit struct.
//! You have to provide the following implementations:
//! - An extension: [`Format::EXTENSION`]
//! - An error for serialization: [`Format::SerializeErr`]
//! - An error for deserialization: [`Format::DeserializeErr`]
//! - A way to deserialize from a slice: [`Format::from_slice`]
//! - A way to serialize to a vec: [`Format::to_vec`]
//!
//! # Example of using this crate
//! ```rust,no_run
//! // When you import this trait, it provides `save_to_file` and `load_from_file`
//! // for any type that implements serde's `Serialize` and `Deserialize`
//! use persist::PersistExt as _;
//!
//! // A format you want to use
//! use persist::json::Json;
//!
//! #[derive(::serde::Serialize, ::serde::Deserialize)]
//! struct Foo {
//!     count: i32,
//!     list: Vec<String>
//!     #[serde(ignore)],
//!     secret: Option<i32>
//! }
//!
//! let foo = Foo {
//!     count: 42,
//!     list: vec![String::from("hello")],
//!     secret: None
//! };
//!
//! // this saves a type to a file called `my_important_type.json`
//! // note: the extension is provided for you, based on the `Format` impl.
//! foo.save_to_file::<Json>("my_important_type")?;
//! // this loads a type from the same file, `my_important_type.json`
//! // note: the extension is provided for you, based on the `Format` impl.
//! let bar = Foo::load_from_file::<Json>("my_important_type")?;
//!
//! // Don't want to save to a file?
//! // You can also save/load to a `std::io::Read` and `std::io::Write` directly
//! use persist::Persist as _;
//!
//! let mut data = vec![];
//! // this saves the type to a vec (an implementation of `std::io::Write`)
//! foo.save::<Json>(&mut data)?;
//! // this loads the type from a slice (an implementation of `std::io::Read`)
//! let bar = Foo::load::<Json>(&data)?;
//! ```

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// The base [`Persist`] trait, this provides both [`Persist::save`] and [`Persist::load`] for the provided [`Format`]
///
/// This gets implemented for every type that implements [`serde::Serialize`] and [`serde::Deserialize`]
pub trait Persist {
    /// Serialize this type with the provided [`Format`] to the writer
    fn save<K>(&self, mut output: impl Write + Sized) -> Result<(), K::SerializeErr>
    where
        K: Format,
        Self: Serialize,
    {
        let data = K::to_vec(self)?;
        output.write_all(&data).map_err(serde::ser::Error::custom)
    }

    /// Serialize this type with the provided [`Format`] from the reader
    fn load<K>(mut input: impl Read + Sized) -> Result<Self, K::DeserializeErr>
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

impl<T> Persist for T where T: for<'de> Deserialize<'de> + Serialize {}

mod format;
pub use format::Format;

mod ext;
pub use ext::PersistExt;

#[cfg(feature = "tokio")]
pub mod tokio;

mod formats {
    #[cfg(feature = "json")]
    pub mod json;

    #[cfg(feature = "yaml")]
    pub mod yaml;

    #[cfg(feature = "toml")]
    pub mod toml;

    #[cfg(feature = "ron")]
    pub mod ron;
}

pub use formats::*;
