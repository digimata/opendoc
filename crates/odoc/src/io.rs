use std::io;
use std::path::{Path, PathBuf};

use crate::Document;

// ---------------------------------
// projects/opendoc/crates/odoc/src/io.rs
//
// pub enum ReadError            L24
// pub enum WriteError           L38
// pub fn from_str()             L50
// pub fn from_slice()           L54
// pub fn from_reader()          L58
// pub fn from_path()            L62
// pub fn to_string()            L76
// pub fn to_string_pretty()     L80
// pub fn to_writer()            L84
// pub fn to_writer_pretty()     L88
// pub fn to_path()              L92
// pub fn to_path_pretty()      L102
// ---------------------------------

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("io error reading {path:?}: {source}")]
    Io {
        path: Option<PathBuf>,
        source: io::Error,
    },
    #[error("json parse error in {path:?}: {source}")]
    Json {
        path: Option<PathBuf>,
        source: serde_json::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("io error writing {path:?}: {source}")]
    Io {
        path: Option<PathBuf>,
        source: io::Error,
    },
    #[error("json serialization error: {source}")]
    Json { source: serde_json::Error },
}

// ── Read ──

pub fn from_str(input: &str) -> Result<Document, ReadError> {
    serde_json::from_str(input).map_err(|source| ReadError::Json { path: None, source })
}

pub fn from_slice(input: &[u8]) -> Result<Document, ReadError> {
    serde_json::from_slice(input).map_err(|source| ReadError::Json { path: None, source })
}

pub fn from_reader<R: io::Read>(reader: R) -> Result<Document, ReadError> {
    serde_json::from_reader(reader).map_err(|source| ReadError::Json { path: None, source })
}

pub fn from_path(path: impl AsRef<Path>) -> Result<Document, ReadError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|source| ReadError::Io {
        path: Some(path.to_path_buf()),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| ReadError::Json {
        path: Some(path.to_path_buf()),
        source,
    })
}

// ── Write ──

pub fn to_string(doc: &Document) -> Result<String, WriteError> {
    serde_json::to_string(doc).map_err(|source| WriteError::Json { source })
}

pub fn to_string_pretty(doc: &Document) -> Result<String, WriteError> {
    serde_json::to_string_pretty(doc).map_err(|source| WriteError::Json { source })
}

pub fn to_writer<W: io::Write>(writer: W, doc: &Document) -> Result<(), WriteError> {
    serde_json::to_writer(writer, doc).map_err(|source| WriteError::Json { source })
}

pub fn to_writer_pretty<W: io::Write>(writer: W, doc: &Document) -> Result<(), WriteError> {
    serde_json::to_writer_pretty(writer, doc).map_err(|source| WriteError::Json { source })
}

pub fn to_path(path: impl AsRef<Path>, doc: &Document) -> Result<(), WriteError> {
    let path = path.as_ref();
    let file = std::fs::File::create(path).map_err(|source| WriteError::Io {
        path: Some(path.to_path_buf()),
        source,
    })?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, doc).map_err(|source| WriteError::Json { source })
}

pub fn to_path_pretty(path: impl AsRef<Path>, doc: &Document) -> Result<(), WriteError> {
    let path = path.as_ref();
    let file = std::fs::File::create(path).map_err(|source| WriteError::Io {
        path: Some(path.to_path_buf()),
        source,
    })?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, doc).map_err(|source| WriteError::Json { source })
}
