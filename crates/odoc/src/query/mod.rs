mod helpers;

use std::collections::HashMap;

use crate::{Annotation, Block, Document, InlineNode, ListItem, ProvenanceRecord};

// ----------------------------------------
// projects/opendoc/crates/odoc/src/query/mod.rs
//
// mod helpers                           L1
// pub enum NodeRef                     L28
// pub enum IndexError                  L36
// pub struct DocumentIndex             L52
//   pub fn new()                       L62
//   pub fn node()                     L112
//   pub fn block()                    L116
//   pub fn inline()                   L123
//   pub fn list_item()                L130
//   pub fn annotation()               L137
//   pub fn provenance()               L141
//   pub fn annotations_targeting()    L145
// fn index_blocks()                   L151
// fn index_inlines()                  L192
// ----------------------------------------

/// Borrowed reference to any addressable content node.
#[derive(Debug, Clone, Copy)]
pub enum NodeRef<'a> {
    Block(&'a Block),
    Inline(&'a InlineNode),
    ListItem(&'a ListItem),
}

/// Errors found while building a `DocumentIndex`.
#[derive(Debug, Clone, thiserror::Error)]
pub enum IndexError {
    #[error("duplicate node ID: {id}")]
    DuplicateNodeId { id: String },
    #[error("duplicate annotation ID: {id}")]
    DuplicateAnnotationId { id: String },
    #[error("duplicate provenance ID: {id}")]
    DuplicateProvenanceId { id: String },
    #[error("annotation {annotation_id} targets missing node {node_id}")]
    MissingTargetNode {
        annotation_id: String,
        node_id: String,
    },
}

/// Pre-built lookup index for O(1) node/annotation access.
#[derive(Debug)]
pub struct DocumentIndex<'a> {
    nodes: HashMap<&'a str, NodeRef<'a>>,
    annotations: HashMap<&'a str, &'a Annotation>,
    provenance: HashMap<&'a str, &'a ProvenanceRecord>,
    /// `annotation_id` → annotation for each annotation targeting a given `node_id`.
    targets: HashMap<&'a str, Vec<&'a Annotation>>,
}

impl<'a> DocumentIndex<'a> {
    /// Build an index over the document, validating ID uniqueness and annotation targets.
    pub fn new(doc: &'a Document) -> Result<Self, IndexError> {
        let mut nodes = HashMap::new();
        let mut annotations = HashMap::new();
        let mut provenance = HashMap::new();
        let mut targets: HashMap<&'a str, Vec<&'a Annotation>> = HashMap::new();

        // Index all content nodes.
        index_blocks(&doc.content, &mut nodes)?;

        // Index annotations.
        if let Some(ref anns) = doc.annotations {
            for ann in anns {
                if annotations.contains_key(ann.id.as_str()) {
                    return Err(IndexError::DuplicateAnnotationId { id: ann.id.clone() });
                }
                annotations.insert(ann.id.as_str(), ann);

                // Validate targets exist and build reverse index.
                for node_id in ann.target.node_ids() {
                    if !nodes.contains_key(node_id) {
                        return Err(IndexError::MissingTargetNode {
                            annotation_id: ann.id.clone(),
                            node_id: node_id.to_string(),
                        });
                    }
                    targets.entry(node_id).or_default().push(ann);
                }
            }
        }

        // Index provenance.
        if let Some(ref provs) = doc.provenance {
            for prov in provs {
                if provenance.contains_key(prov.id.as_str()) {
                    return Err(IndexError::DuplicateProvenanceId {
                        id: prov.id.clone(),
                    });
                }
                provenance.insert(prov.id.as_str(), prov);
            }
        }

        Ok(Self {
            nodes,
            annotations,
            provenance,
            targets,
        })
    }

    pub fn node(&self, id: &str) -> Option<NodeRef<'a>> {
        self.nodes.get(id).copied()
    }

    pub fn block(&self, id: &str) -> Option<&'a Block> {
        match self.nodes.get(id)? {
            NodeRef::Block(b) => Some(b),
            _ => None,
        }
    }

    pub fn inline(&self, id: &str) -> Option<&'a InlineNode> {
        match self.nodes.get(id)? {
            NodeRef::Inline(i) => Some(i),
            _ => None,
        }
    }

    pub fn list_item(&self, id: &str) -> Option<&'a ListItem> {
        match self.nodes.get(id)? {
            NodeRef::ListItem(li) => Some(li),
            _ => None,
        }
    }

    pub fn annotation(&self, id: &str) -> Option<&'a Annotation> {
        self.annotations.get(id).copied()
    }

    pub fn provenance(&self, id: &str) -> Option<&'a ProvenanceRecord> {
        self.provenance.get(id).copied()
    }

    pub fn annotations_targeting(&self, node_id: &str) -> &[&'a Annotation] {
        self.targets.get(node_id).map_or(&[], |v| v.as_slice())
    }
}

/// Recursively index all blocks and their inline/list-item children.
fn index_blocks<'a>(
    blocks: &'a [Block],
    nodes: &mut HashMap<&'a str, NodeRef<'a>>,
) -> Result<(), IndexError> {
    for block in blocks {
        let id = block.id();
        if nodes.contains_key(id) {
            return Err(IndexError::DuplicateNodeId {
                id: id.to_string(),
            });
        }
        nodes.insert(id, NodeRef::Block(block));

        match block {
            Block::Paragraph { content, .. } | Block::Heading { content, .. } => {
                index_inlines(content, nodes)?;
            }
            Block::List { items, .. } => {
                for item in items {
                    if nodes.contains_key(item.id.as_str()) {
                        return Err(IndexError::DuplicateNodeId {
                            id: item.id.clone(),
                        });
                    }
                    nodes.insert(item.id.as_str(), NodeRef::ListItem(item));
                    index_blocks(&item.content, nodes)?;
                }
            }
            Block::Blockquote { content, .. } => {
                index_blocks(content, nodes)?;
            }
            Block::Code { .. }
            | Block::Table { .. }
            | Block::Image { .. }
            | Block::Divider { .. }
            | Block::Embed { .. } => {}
        }
    }
    Ok(())
}

fn index_inlines<'a>(
    inlines: &'a [InlineNode],
    nodes: &mut HashMap<&'a str, NodeRef<'a>>,
) -> Result<(), IndexError> {
    for inline in inlines {
        let id = inline.id();
        if nodes.contains_key(id) {
            return Err(IndexError::DuplicateNodeId {
                id: id.to_string(),
            });
        }
        nodes.insert(id, NodeRef::Inline(inline));
    }
    Ok(())
}
