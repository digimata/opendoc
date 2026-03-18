#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

pub mod io;
pub mod query;
// -----------------------------
// projects/opendoc/crates/odoc/src/lib.rs
//
// pub mod io                 L3
// pub mod query              L4
// pub mod render            L17
// pub mod types             L18
// pub const ODOC_VERSION    L25
// pub const SCHEMA_URI      L26
// -----------------------------

#[cfg(feature = "convert")]
pub mod convert;
#[cfg(feature = "render")]
pub mod render;
pub mod types;

pub use types::{
    Annotation, Block, BoundaryPoint, Context, Document, InlineNode, ListItem, Mark, Meta,
    MetaSource, ProvenanceBody, ProvenanceKind, ProvenanceRecord, Target,
};

pub const ODOC_VERSION: &str = "1.0.0";
pub const SCHEMA_URI: &str = "https://opendoc.dev/schema/odoc/1.0.0.json";
