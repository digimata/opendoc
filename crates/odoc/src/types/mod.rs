mod annotation;
mod block;
mod inline;
mod provenance;

pub use annotation::{Annotation, BoundaryPoint, Target};
pub use block::{Block, ListItem};
pub use inline::{InlineNode, Mark};
pub use provenance::{ProvenanceBody, ProvenanceKind, ProvenanceRecord};

use serde::{Deserialize, Serialize};

/// Top-level `OpenDoc` document envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub odoc: String,
    pub id: String,
    pub meta: Meta,
    pub content: Vec<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@context")]
    pub context: Option<Context>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<Vec<ProvenanceRecord>>,
}

/// Document metadata. `title` and `created_at` are required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MetaSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Source metadata for the original artifact this document was produced from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaSource {
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<String>,
}

/// JSON-LD context — can be a single string or an array of strings/objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Context {
    Single(String),
    Multiple(Vec<serde_json::Value>),
}
