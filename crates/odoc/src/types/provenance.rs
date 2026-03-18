use serde::{Deserialize, Serialize};

// ----------------------------------
// projects/opendoc/crates/odoc/src/types/provenance.rs
//
// pub struct ProvenanceRecord    L13
// pub enum ProvenanceKind        L22
// pub struct ProvenanceBody      L33
// ----------------------------------

/// A provenance record — tracks where content or annotations came from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: String,
    pub kind: ProvenanceKind,
    pub body: ProvenanceBody,
}

/// Core provenance categories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceKind {
    /// Where content came from in the original artifact.
    Source,
    /// How a semantic claim or annotation was produced.
    Assertion,
    /// How content or annotations were computed from earlier records.
    Derived,
}

/// Common fields for provenance record bodies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_refs: Option<Vec<String>>,
}
