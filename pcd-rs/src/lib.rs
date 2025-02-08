//! Read and write PCD file format.
//!
//! This crate provides data serializer and deserializer for PCD
//! (Point Cloud Data) file format. The [DynReader] and [DynWriter]
//! can read and write PCD files with any valid schemas. It also
//! supports deserializing to static types if the `derive` feature is
//! enabled.
//!
//! # Supported Format Versions
//!
//! - 0.7
//! - Older versions are not supported yet.

pub use byteorder;

pub mod error;
pub mod metas;
pub mod prelude;
pub mod reader;
pub mod record;
pub mod traits;
mod utils;
pub mod writer;

pub use error::{Error, Result};
pub use metas::{DataKind, FieldDef, PcdMeta, Schema, TypeKind, ValueKind, ViewPoint};
pub use bye_pcd_derive_rs::{PcdDeserialize, PcdSerialize};
pub use reader::{DynReader, Reader};
pub use record::{DynRecord, Field, PcdDeserialize, PcdSerialize};
pub use traits::Value;
pub use writer::{DynWriter, Writer, WriterInit};
