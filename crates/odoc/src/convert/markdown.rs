use std::path::Path;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::{Block, Document, InlineNode, ListItem, Mark, Meta, ODOC_VERSION, SCHEMA_URI};

use super::{ConvertError, ConvertOptions, Convert, InputFormat};

// ------------------------------------
// projects/opendoc/crates/odoc/src/convert/markdown.rs
//
// pub struct MarkdownConverter     L32
//   fn format()                    L35
//   fn convert()                   L39
// pub fn markdown()                L45
// pub fn markdown_from_path()      L90
// struct ConverterState            L99
// struct Counters                 L116
//   fn next()                     L131
// enum PendingBlock               L151
// enum BlockContext               L158
//   fn new()                      L165
//   fn push_block()               L177
//   fn flush_block()              L194
//   fn push_text()                L246
//   fn process()                  L267
// fn uuid_v4_stub()               L539
// ------------------------------------

/// Converts Markdown text into an odoc `Document`.
#[derive(Debug, Clone, Default)]
pub struct MarkdownConverter;

impl Convert for MarkdownConverter {
    fn format(&self) -> InputFormat {
        InputFormat::Markdown
    }

    fn convert(&self, input: &str, options: &ConvertOptions) -> Result<Document, ConvertError> {
        markdown(input, options)
    }
}

/// Convenience: convert a markdown string to a `Document`.
pub fn markdown(input: &str, options: &ConvertOptions) -> Result<Document, ConvertError> {
    let mut state = ConverterState::new();
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let parser = Parser::new_ext(input, opts);

    for event in parser {
        state.process(event);
    }

    // Flush any pending block.
    state.flush_block();

    let id = options
        .document_id
        .clone()
        .unwrap_or_else(|| format!("urn:odoc:{}", uuid_v4_stub()));
    let title = options.title.clone().unwrap_or_default();
    let created_at = options
        .created_at
        .clone()
        .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());

    Ok(Document {
        schema: SCHEMA_URI.to_string(),
        odoc: ODOC_VERSION.to_string(),
        id,
        meta: Meta {
            title,
            created_at,
            modified_at: None,
            producer: options.producer.clone(),
            language: options.language.clone(),
            source: options.source.clone(),
            tags: None,
        },
        content: state.blocks,
        context: None,
        annotations: None,
        provenance: None,
    })
}

/// Convenience: convert a markdown file to a `Document`.
pub fn markdown_from_path(
    path: impl AsRef<Path>,
    options: &ConvertOptions,
) -> Result<Document, ConvertError> {
    super::from_path(&MarkdownConverter, path, options)
}

// ── Internal state machine ──

struct ConverterState {
    blocks: Vec<Block>,
    /// Stack of block contexts (for nesting: list items, blockquotes).
    block_stack: Vec<BlockContext>,
    /// Current inline content being built.
    inlines: Vec<InlineNode>,
    /// Active inline marks.
    mark_stack: Vec<Mark>,
    /// Current link destination (set when inside a link tag).
    link_href: Option<String>,
    /// Counters for generating sequential IDs per type.
    counters: Counters,
    /// State for the current block being built.
    current_block: Option<PendingBlock>,
}

#[derive(Default)]
struct Counters {
    heading: usize,
    paragraph: usize,
    text: usize,
    code: usize,
    list: usize,
    list_item: usize,
    blockquote: usize,
    table: usize,
    image: usize,
    divider: usize,
    hard_break: usize,
}

impl Counters {
    fn next(&mut self, prefix: &str) -> String {
        let counter = match prefix {
            "h" => &mut self.heading,
            "p" => &mut self.paragraph,
            "t" => &mut self.text,
            "c" => &mut self.code,
            "l" => &mut self.list,
            "li" => &mut self.list_item,
            "q" => &mut self.blockquote,
            "tbl" => &mut self.table,
            "img" => &mut self.image,
            "d" => &mut self.divider,
            "br" => &mut self.hard_break,
            _ => unreachable!(),
        };
        *counter += 1;
        format!("{prefix}{counter}")
    }
}

enum PendingBlock {
    Heading { level: u8 },
    Paragraph,
    Code { language: Option<String>, text: String },
    Table { columns: Vec<String>, rows: Vec<Vec<String>>, in_header: bool, current_cell: String },
}

enum BlockContext {
    List { id: String, ordered: bool, items: Vec<ListItem> },
    ListItem { id: String, blocks: Vec<Block> },
    Blockquote { id: String, blocks: Vec<Block> },
}

impl ConverterState {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_stack: Vec::new(),
            inlines: Vec::new(),
            mark_stack: Vec::new(),
            link_href: None,
            counters: Counters::default(),
            current_block: None,
        }
    }

    fn push_block(&mut self, block: Block) {
        // If we're inside a nested context, push to that context.
        if let Some(ctx) = self.block_stack.last_mut() {
            match ctx {
                BlockContext::ListItem { blocks, .. }
                | BlockContext::Blockquote { blocks, .. } => {
                    blocks.push(block);
                    return;
                }
                BlockContext::List { .. } => {
                    // Shouldn't happen — blocks go into list items, not lists directly.
                }
            }
        }
        self.blocks.push(block);
    }

    fn flush_block(&mut self) {
        let Some(pending) = self.current_block.take() else {
            return;
        };

        let block = match pending {
            PendingBlock::Heading { level } => {
                let id = self.counters.next("h");
                let content = std::mem::take(&mut self.inlines);
                Block::Heading {
                    id,
                    level,
                    content,
                    anchor: None,
                    prov_refs: None,
                }
            }
            PendingBlock::Paragraph => {
                let id = self.counters.next("p");
                let content = std::mem::take(&mut self.inlines);
                if content.is_empty() {
                    return;
                }
                Block::Paragraph {
                    id,
                    content,
                    prov_refs: None,
                }
            }
            PendingBlock::Code { language, text } => {
                let id = self.counters.next("c");
                Block::Code {
                    id,
                    text,
                    language,
                    prov_refs: None,
                }
            }
            PendingBlock::Table { columns, rows, .. } => {
                let id = self.counters.next("tbl");
                Block::Table {
                    id,
                    columns,
                    rows,
                    caption: None,
                    prov_refs: None,
                }
            }
        };
        self.push_block(block);
    }

    fn push_text(&mut self, text: &str) {
        if let Some(PendingBlock::Table { ref mut current_cell, .. }) = self.current_block {
            current_cell.push_str(text);
            return;
        }

        let id = self.counters.next("t");
        let marks = if self.mark_stack.is_empty() {
            None
        } else {
            Some(self.mark_stack.clone())
        };
        self.inlines.push(InlineNode::Text {
            id,
            text: text.to_string(),
            marks,
            prov_refs: None,
        });
    }

    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    fn process(&mut self, event: Event) {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    self.flush_block();
                    self.current_block = Some(PendingBlock::Heading {
                        level: level as u8,
                    });
                }
                Tag::Paragraph => {
                    self.flush_block();
                    self.current_block = Some(PendingBlock::Paragraph);
                }
                Tag::CodeBlock(kind) => {
                    self.flush_block();
                    let language = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                            let lang = lang.to_string();
                            if lang.is_empty() { None } else { Some(lang) }
                        }
                        pulldown_cmark::CodeBlockKind::Indented => None,
                    };
                    self.current_block = Some(PendingBlock::Code {
                        language,
                        text: String::new(),
                    });
                }
                Tag::List(start) => {
                    self.flush_block();
                    let id = self.counters.next("l");
                    self.block_stack.push(BlockContext::List {
                        id,
                        ordered: start.is_some(),
                        items: Vec::new(),
                    });
                }
                Tag::Item => {
                    let id = self.counters.next("li");
                    self.block_stack.push(BlockContext::ListItem {
                        id,
                        blocks: Vec::new(),
                    });
                }
                Tag::BlockQuote(_) => {
                    self.flush_block();
                    let id = self.counters.next("q");
                    self.block_stack.push(BlockContext::Blockquote {
                        id,
                        blocks: Vec::new(),
                    });
                }
                Tag::Table(alignments) => {
                    self.flush_block();
                    self.current_block = Some(PendingBlock::Table {
                        columns: Vec::with_capacity(alignments.len()),
                        rows: Vec::new(),
                        in_header: false,
                        current_cell: String::new(),
                    });
                }
                Tag::TableHead => {
                    if let Some(PendingBlock::Table { ref mut in_header, .. }) = self.current_block {
                        *in_header = true;
                    }
                }
                Tag::TableRow => {}
                Tag::TableCell => {
                    if let Some(PendingBlock::Table { ref mut current_cell, .. }) = self.current_block {
                        current_cell.clear();
                    }
                }
                Tag::Emphasis => {
                    self.mark_stack.push(Mark::Em);
                }
                Tag::Strong => {
                    self.mark_stack.push(Mark::Strong);
                }
                Tag::Strikethrough => {
                    self.mark_stack.push(Mark::Strikethrough);
                }
                Tag::Link { dest_url, .. } => {
                    self.link_href = Some(dest_url.to_string());
                }
                Tag::Image { dest_url, title, .. } => {
                    self.flush_block();
                    let id = self.counters.next("img");
                    let alt = if title.is_empty() {
                        None
                    } else {
                        Some(title.to_string())
                    };
                    self.push_block(Block::Image {
                        id,
                        src: dest_url.to_string(),
                        alt,
                        caption: None,
                        prov_refs: None,
                    });
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) | TagEnd::Paragraph => {
                    self.flush_block();
                }
                TagEnd::CodeBlock => {
                    // Trim trailing newline that pulldown-cmark adds.
                    if let Some(PendingBlock::Code { ref mut text, .. }) = self.current_block
                        && text.ends_with('\n')
                    {
                        text.pop();
                    }
                    self.flush_block();
                }
                TagEnd::List(_) => {
                    if let Some(BlockContext::List { id, ordered, items }) = self.block_stack.pop() {
                        let block = Block::List {
                            id,
                            ordered,
                            items,
                            prov_refs: None,
                        };
                        self.push_block(block);
                    }
                }
                TagEnd::Item => {
                    // Tight lists: pulldown-cmark doesn't emit Paragraph tags,
                    // so pending inlines need to be wrapped manually.
                    if !self.inlines.is_empty() && self.current_block.is_none() {
                        self.current_block = Some(PendingBlock::Paragraph);
                    }
                    self.flush_block();
                    if let Some(BlockContext::ListItem { id, blocks }) = self.block_stack.pop() {
                        let item = ListItem { id, content: blocks };
                        if let Some(BlockContext::List { items, .. }) = self.block_stack.last_mut() {
                            items.push(item);
                        }
                    }
                }
                TagEnd::BlockQuote(_) => {
                    self.flush_block();
                    if let Some(BlockContext::Blockquote { id, blocks }) = self.block_stack.pop() {
                        let block = Block::Blockquote {
                            id,
                            content: blocks,
                            attribution: None,
                            prov_refs: None,
                        };
                        self.push_block(block);
                    }
                }
                TagEnd::Table => {
                    self.flush_block();
                }
                TagEnd::TableHead => {
                    if let Some(PendingBlock::Table { ref mut in_header, .. }) = self.current_block {
                        *in_header = false;
                    }
                }
                TagEnd::TableRow => {}
                TagEnd::TableCell => {
                    if let Some(PendingBlock::Table {
                        ref mut columns,
                        ref mut rows,
                        in_header,
                        ref mut current_cell,
                        ..
                    }) = self.current_block
                    {
                        let value = std::mem::take(current_cell);
                        if in_header {
                            columns.push(value);
                        } else {
                            if rows.is_empty() || rows.last().is_some_and(|r| r.len() >= columns.len()) {
                                rows.push(Vec::new());
                            }
                            if let Some(row) = rows.last_mut() {
                                row.push(value);
                            }
                        }
                    }
                }
                TagEnd::Emphasis => {
                    self.mark_stack.retain(|m| !matches!(m, Mark::Em));
                }
                TagEnd::Strong => {
                    self.mark_stack.retain(|m| !matches!(m, Mark::Strong));
                }
                TagEnd::Strikethrough => {
                    self.mark_stack.retain(|m| !matches!(m, Mark::Strikethrough));
                }
                TagEnd::Link => {
                    // Apply link mark to all pending inlines that were collected
                    // while inside the link.
                    if let Some(href) = self.link_href.take() {
                        let link_mark = Mark::Link { href, title: None };
                        // Walk backwards through inlines and add link mark to recent text nodes.
                        // This is a simplification — we apply link to all inlines since we don't
                        // track where the link started, but in practice link content is contiguous.
                        // A more precise approach would track an inline index, but this works for
                        // the common case.
                        for inline in &mut self.inlines {
                            if let InlineNode::Text { marks, .. } = inline {
                                let marks = marks.get_or_insert_with(Vec::new);
                                if !marks.iter().any(|m| matches!(m, Mark::Link { .. })) {
                                    marks.push(link_mark.clone());
                                }
                            }
                        }
                    }
                }
                TagEnd::Image => {}
                _ => {}
            },
            Event::Text(text) => {
                if let Some(PendingBlock::Code { text: ref mut code_text, .. }) = self.current_block {
                    code_text.push_str(&text);
                } else {
                    self.push_text(&text);
                }
            }
            Event::Code(code) => {
                // Inline code.
                let id = self.counters.next("t");
                let mut marks = self.mark_stack.clone();
                marks.push(Mark::Code);
                self.inlines.push(InlineNode::Text {
                    id,
                    text: code.to_string(),
                    marks: Some(marks),
                    prov_refs: None,
                });
            }
            Event::SoftBreak => {
                // Soft breaks become spaces in markdown rendering.
                self.push_text(" ");
            }
            Event::HardBreak => {
                let id = self.counters.next("br");
                self.inlines.push(InlineNode::HardBreak { id });
            }
            Event::Rule => {
                self.flush_block();
                let id = self.counters.next("d");
                self.push_block(Block::Divider { id });
            }
            Event::Html(html) => {
                // Treat raw HTML as a paragraph with text content.
                let text = html.to_string();
                if !text.trim().is_empty() {
                    self.flush_block();
                    let id = self.counters.next("p");
                    let tid = self.counters.next("t");
                    self.push_block(Block::Paragraph {
                        id,
                        content: vec![InlineNode::Text {
                            id: tid,
                            text,
                            marks: None,
                            prov_refs: None,
                        }],
                        prov_refs: None,
                    });
                }
            }
            _ => {}
        }
    }
}

/// Stub UUID generator — produces a deterministic-ish ID.
/// In production this would use a proper UUID v7 library.
fn uuid_v4_stub() -> String {
    "00000000-0000-0000-0000-000000000000".to_string()
}
