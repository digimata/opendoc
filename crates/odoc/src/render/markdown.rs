use std::fmt;

use crate::{Block, Document, InlineNode, Mark};

use super::{OutputFormat, Render, RenderError};

// -----------------------------------
// projects/opendoc/crates/odoc/src/render/markdown.rs
//
// pub struct MarkdownRenderer     L25
//   fn format()                   L28
//   fn render_into()              L32
// pub fn markdown()               L39
// fn render_blocks()              L43
// fn render_block()               L58
// fn render_table()              L143
// fn render_list_item_block()    L180
// fn render_inlines()            L200
// fn render_inline()             L207
// fn render_marked_text()        L221
// -----------------------------------

/// Renders an odoc `Document` to GitHub-Flavored Markdown.
#[derive(Debug, Clone, Default)]
pub struct MarkdownRenderer;

impl Render for MarkdownRenderer {
    fn format(&self) -> OutputFormat {
        OutputFormat::Markdown
    }

    fn render_into(&self, doc: &Document, out: &mut dyn fmt::Write) -> Result<(), RenderError> {
        render_blocks(&doc.content, out, "")?;
        Ok(())
    }
}

/// Convenience: render document to markdown string.
pub fn markdown(doc: &Document) -> Result<String, RenderError> {
    MarkdownRenderer.render(doc)
}

fn render_blocks(
    blocks: &[Block],
    out: &mut dyn fmt::Write,
    prefix: &str,
) -> Result<(), fmt::Error> {
    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            // Blank line between blocks (with prefix for blockquotes).
            writeln!(out, "{prefix}")?;
        }
        render_block(block, out, prefix)?;
    }
    Ok(())
}

fn render_block(block: &Block, out: &mut dyn fmt::Write, prefix: &str) -> Result<(), fmt::Error> {
    match block {
        Block::Heading { level, content, .. } => {
            write!(out, "{prefix}")?;
            for _ in 0..*level {
                write!(out, "#")?;
            }
            write!(out, " ")?;
            render_inlines(content, out)?;
            writeln!(out)?;
        }
        Block::Paragraph { content, .. } => {
            write!(out, "{prefix}")?;
            render_inlines(content, out)?;
            writeln!(out)?;
        }
        Block::List { ordered, items, .. } => {
            for (idx, item) in items.iter().enumerate() {
                let bullet = if *ordered {
                    format!("{}. ", idx + 1)
                } else {
                    "- ".to_string()
                };
                let indent = " ".repeat(bullet.len());

                for (bi, block) in item.content.iter().enumerate() {
                    if bi == 0 {
                        // First block gets the bullet.
                        let item_prefix = format!("{prefix}{bullet}");
                        render_list_item_block(block, out, &item_prefix, prefix, &indent)?;
                    } else {
                        // Subsequent blocks get continuation indent.
                        writeln!(out, "{prefix}")?;
                        let cont_prefix = format!("{prefix}{indent}");
                        render_block(block, out, &cont_prefix)?;
                    }
                }
            }
        }
        Block::Blockquote { content, .. } => {
            let new_prefix = format!("{prefix}> ");
            render_blocks(content, out, &new_prefix)?;
        }
        Block::Code { text, language, .. } => {
            let lang = language.as_deref().unwrap_or("");
            writeln!(out, "{prefix}```{lang}")?;
            for line in text.lines() {
                writeln!(out, "{prefix}{line}")?;
            }
            // Handle trailing newline: if text ends with \n, lines() already covered it.
            // If text is empty or doesn't end with \n, we still need the fence.
            writeln!(out, "{prefix}```")?;
        }
        Block::Table {
            columns,
            rows,
            caption,
            ..
        } => {
            render_table(columns, rows, caption.as_deref(), out, prefix)?;
        }
        Block::Image {
            src, alt, caption, ..
        } => {
            let alt_text = alt.as_deref().unwrap_or("");
            writeln!(out, "{prefix}![{alt_text}]({src})")?;
            if let Some(caption) = caption {
                writeln!(out)?;
                writeln!(out, "{prefix}*{caption}*")?;
            }
        }
        Block::Divider { .. } => {
            writeln!(out, "{prefix}---")?;
        }
        Block::Embed { src, title, .. } => {
            if let Some(title) = title {
                writeln!(out, "{prefix}[{title}]({src})")?;
            } else {
                writeln!(out, "{prefix}{src}")?;
            }
        }
    }
    Ok(())
}

fn render_table(
    columns: &[String],
    rows: &[Vec<String>],
    caption: Option<&str>,
    out: &mut dyn fmt::Write,
    prefix: &str,
) -> Result<(), fmt::Error> {
    write!(out, "{prefix}|")?;
    for col in columns {
        write!(out, " {col} |")?;
    }
    writeln!(out)?;

    write!(out, "{prefix}|")?;
    for col in columns {
        let dashes = "-".repeat(col.len().max(3));
        write!(out, " {dashes} |")?;
    }
    writeln!(out)?;

    for row in rows {
        write!(out, "{prefix}|")?;
        for (i, cell) in row.iter().enumerate() {
            let col_width = columns.get(i).map_or(3, |c| c.len().max(3));
            write!(out, " {cell:<col_width$} |")?;
        }
        writeln!(out)?;
    }

    if let Some(caption) = caption {
        writeln!(out)?;
        writeln!(out, "{prefix}*{caption}*")?;
    }
    Ok(())
}

/// Render the first block of a list item (which gets the bullet prefix).
fn render_list_item_block(
    block: &Block,
    out: &mut dyn fmt::Write,
    bullet_prefix: &str,
    base_prefix: &str,
    indent: &str,
) -> Result<(), fmt::Error> {
    if let Block::Paragraph { content, .. } = block {
        write!(out, "{bullet_prefix}")?;
        render_inlines(content, out)?;
        writeln!(out)?;
    } else {
        let cont_prefix = format!("{base_prefix}{indent}");
        write!(out, "{bullet_prefix}")?;
        writeln!(out)?;
        render_block(block, out, &cont_prefix)?;
    }
    Ok(())
}

fn render_inlines(inlines: &[InlineNode], out: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
    for inline in inlines {
        render_inline(inline, out)?;
    }
    Ok(())
}

fn render_inline(inline: &InlineNode, out: &mut dyn fmt::Write) -> Result<(), fmt::Error> {
    match inline {
        InlineNode::Text { text, marks, .. } => {
            let marks = marks.as_deref().unwrap_or_default();
            render_marked_text(text, marks, out)?;
        }
        InlineNode::HardBreak { .. } => {
            // Two trailing spaces + newline for markdown hard break.
            writeln!(out, "  ")?;
        }
    }
    Ok(())
}

fn render_marked_text(
    text: &str,
    marks: &[Mark],
    out: &mut dyn fmt::Write,
) -> Result<(), fmt::Error> {
    // Find if there's a link mark — it wraps the whole text.
    let link = marks.iter().find_map(|m| match m {
        Mark::Link { href, .. } => Some(href.as_str()),
        _ => None,
    });
    let footnote = marks.iter().find_map(|m| match m {
        Mark::Footnote { note } => Some(note.as_str()),
        _ => None,
    });

    if link.is_some() {
        write!(out, "[")?;
    }

    // Apply opening marks (innermost first doesn't matter for markdown).
    for mark in marks {
        match mark {
            Mark::Strong => write!(out, "**")?,
            Mark::Em => write!(out, "*")?,
            Mark::Code => write!(out, "`")?,
            Mark::Strikethrough => write!(out, "~~")?,
            // These have no standard markdown representation — pass through.
            Mark::Underline
            | Mark::Sup
            | Mark::Sub
            | Mark::Highlight { .. }
            | Mark::Link { .. }
            | Mark::Footnote { .. } => {}
        }
    }

    write!(out, "{text}")?;

    // Close marks in reverse order.
    for mark in marks.iter().rev() {
        match mark {
            Mark::Strong => write!(out, "**")?,
            Mark::Em => write!(out, "*")?,
            Mark::Code => write!(out, "`")?,
            Mark::Strikethrough => write!(out, "~~")?,
            Mark::Underline
            | Mark::Sup
            | Mark::Sub
            | Mark::Highlight { .. }
            | Mark::Link { .. }
            | Mark::Footnote { .. } => {}
        }
    }

    if let Some(href) = link {
        write!(out, "]({href})")?;
    }

    if let Some(note) = footnote {
        write!(out, "[^{note}]")?;
    }

    Ok(())
}
