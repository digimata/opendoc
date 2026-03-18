use crate::{Annotation, Block, Document, InlineNode, ProvenanceRecord, Target};

use super::{DocumentIndex, IndexError};

// ------------------------------
// projects/opendoc/crates/odoc/src/query/helpers.rs
//
//   pub fn annotations()     L22
//   pub fn provenance()      L26
//   pub fn index()           L30
//   pub fn id()              L36
//   pub fn kind()            L50
//   pub fn prov_refs()       L64
//   pub fn id()              L81
//   pub fn kind()            L87
//   pub fn prov_refs()       L94
//   pub fn prov_refs()      L103
//   pub fn node_ids()       L109
// ------------------------------

impl Document {
    pub fn annotations(&self) -> &[Annotation] {
        self.annotations.as_deref().unwrap_or_default()
    }

    pub fn provenance(&self) -> &[ProvenanceRecord] {
        self.provenance.as_deref().unwrap_or_default()
    }

    pub fn index(&self) -> Result<DocumentIndex<'_>, IndexError> {
        DocumentIndex::new(self)
    }
}

impl Block {
    pub fn id(&self) -> &str {
        match self {
            Self::Paragraph { id, .. }
            | Self::Heading { id, .. }
            | Self::List { id, .. }
            | Self::Blockquote { id, .. }
            | Self::Code { id, .. }
            | Self::Table { id, .. }
            | Self::Image { id, .. }
            | Self::Divider { id }
            | Self::Embed { id, .. } => id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Paragraph { .. } => "paragraph",
            Self::Heading { .. } => "heading",
            Self::List { .. } => "list",
            Self::Blockquote { .. } => "blockquote",
            Self::Code { .. } => "code",
            Self::Table { .. } => "table",
            Self::Image { .. } => "image",
            Self::Divider { .. } => "divider",
            Self::Embed { .. } => "embed",
        }
    }

    pub fn prov_refs(&self) -> &[String] {
        let opt = match self {
            Self::Paragraph { prov_refs, .. }
            | Self::Heading { prov_refs, .. }
            | Self::List { prov_refs, .. }
            | Self::Blockquote { prov_refs, .. }
            | Self::Code { prov_refs, .. }
            | Self::Table { prov_refs, .. }
            | Self::Image { prov_refs, .. }
            | Self::Embed { prov_refs, .. } => prov_refs,
            Self::Divider { .. } => return &[],
        };
        opt.as_deref().unwrap_or_default()
    }
}

impl InlineNode {
    pub fn id(&self) -> &str {
        match self {
            Self::Text { id, .. } | Self::HardBreak { id } => id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Text { .. } => "text",
            Self::HardBreak { .. } => "hard_break",
        }
    }

    pub fn prov_refs(&self) -> &[String] {
        match self {
            Self::Text { prov_refs, .. } => prov_refs.as_deref().unwrap_or_default(),
            Self::HardBreak { .. } => &[],
        }
    }
}

impl Annotation {
    pub fn prov_refs(&self) -> &[String] {
        self.prov_refs.as_deref().unwrap_or_default()
    }
}

impl Target {
    pub fn node_ids(&self) -> Vec<&str> {
        match self {
            Self::Node { node } => vec![node.as_str()],
            Self::TextRange { start, end } => {
                let mut ids = vec![start.node.as_str()];
                if start.node != end.node {
                    ids.push(end.node.as_str());
                }
                ids
            }
            Self::NodeRange { start, end } => vec![start.as_str(), end.as_str()],
        }
    }
}
