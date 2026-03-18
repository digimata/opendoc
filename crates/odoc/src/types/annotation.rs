use serde::{Deserialize, Serialize};

/// A semantic annotation targeting content nodes by ID.
///
/// The annotation body is opaque JSON — its schema depends on the annotation type
/// (e.g. `entities:mention`, `data:value`, `xref:link`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    #[serde(rename = "type")]
    pub annotation_type: String,
    pub target: Target,
    pub body: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prov_refs: Option<Vec<String>>,
}

/// What part of the content tree an annotation targets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Target {
    /// Entire block or inline node.
    #[serde(rename = "node")]
    Node { node: String },
    /// Character range across one or more text nodes.
    #[serde(rename = "text_range")]
    TextRange {
        start: BoundaryPoint,
        end: BoundaryPoint,
    },
    /// Contiguous sequence of sibling nodes.
    #[serde(rename = "node_range")]
    NodeRange { start: String, end: String },
}

/// A boundary point within a text node — zero-indexed character offset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryPoint {
    pub node: String,
    pub offset: usize,
}
