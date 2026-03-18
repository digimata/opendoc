use serde::{Deserialize, Serialize};

/// An inline node within block content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InlineNode {
    #[serde(rename = "text")]
    Text {
        id: String,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        marks: Option<Vec<Mark>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prov_refs: Option<Vec<String>>,
    },
    #[serde(rename = "hard_break")]
    HardBreak { id: String },
}

/// Inline formatting mark. All marks are objects with a `type` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mark {
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "em")]
    Em,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "strikethrough")]
    Strikethrough,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "sup")]
    Sup,
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "link")]
    Link {
        href: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    #[serde(rename = "highlight")]
    Highlight {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color: Option<String>,
    },
    #[serde(rename = "footnote")]
    Footnote { note: String },
}
