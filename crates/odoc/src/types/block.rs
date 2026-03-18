use serde::{Deserialize, Serialize};

use super::InlineNode;

/// A block-level node in the document content tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "paragraph")]
    Paragraph {
        id: String,
        content: Vec<InlineNode>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "heading")]
    Heading {
        id: String,
        level: u8,
        content: Vec<InlineNode>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        anchor: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "list")]
    List {
        id: String,
        ordered: bool,
        items: Vec<ListItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "blockquote")]
    Blockquote {
        id: String,
        content: Vec<Block>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attribution: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "code")]
    Code {
        id: String,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "table")]
    Table {
        id: String,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "image")]
    Image {
        id: String,
        src: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        alt: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "divider")]
    Divider { id: String },
    #[serde(rename = "embed")]
    Embed {
        id: String,
        src: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        media_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
}

/// A list item. Content is always `Vec<Block>` — inline content wraps in a paragraph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    pub id: String,
    pub content: Vec<Block>,
}
