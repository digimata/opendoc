// --------------------------------
// projects/opendoc/crates/odoc/src/convert/mod.rs
//
// pub mod markdown             L14
// pub enum InputFormat         L22
// pub struct ConvertOptions    L28
// pub trait Convert            L37
// pub fn from_reader()         L42
// pub fn from_path()           L54
// pub enum ConvertError        L69
// --------------------------------

#[cfg(feature = "convert-markdown")]
pub mod markdown;

use std::io;
use std::path::{Path, PathBuf};

use crate::{Document, MetaSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Markdown,
    Html,
}

#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    pub document_id: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub language: Option<String>,
    pub producer: Option<String>,
    pub source: Option<MetaSource>,
}

pub trait Convert {
    fn format(&self) -> InputFormat;
    fn convert(&self, input: &str, options: &ConvertOptions) -> Result<Document, ConvertError>;
}

pub fn from_reader<C: Convert + ?Sized, R: io::Read>(
    converter: &C,
    mut reader: R,
    options: &ConvertOptions,
) -> Result<Document, ConvertError> {
    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .map_err(|source| ConvertError::Io { path: None, source })?;
    converter.convert(&input, options)
}

pub fn from_path<C: Convert + ?Sized>(
    converter: &C,
    path: impl AsRef<Path>,
    options: &ConvertOptions,
) -> Result<Document, ConvertError> {
    let path = path.as_ref();
    let input =
        std::fs::read_to_string(path).map_err(|source| ConvertError::Io {
            path: Some(path.to_path_buf()),
            source,
        })?;
    converter.convert(&input, options)
}

#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    #[error("io error reading {path:?}: {source}")]
    Io {
        path: Option<PathBuf>,
        source: io::Error,
    },
    #[error("parse error at {line:?}:{column:?}: {message}")]
    Parse {
        line: Option<usize>,
        column: Option<usize>,
        message: String,
    },
    #[error("invalid metadata field {field}: {message}")]
    InvalidMetadata {
        field: &'static str,
        message: String,
    },
}
