#![cfg(feature = "convert-markdown")]

use odoc::convert::markdown;
use odoc::convert::ConvertOptions;

// ---------------------------------------------------
// projects/opendoc/crates/odoc/tests/convert.rs
//
// fn opts()                                       L26
// fn convert_simple_paragraph()                   L36
// fn convert_heading()                            L43
// fn convert_code_block()                         L54
// fn convert_list()                               L67
// fn convert_ordered_list()                       L80
// fn convert_blockquote()                         L92
// fn convert_divider()                           L100
// fn convert_table()                             L108
// fn convert_inline_marks()                      L122
// fn convert_link()                              L138
// fn convert_hard_break()                        L150
// fn sequential_ids_are_stable()                 L159
// fn roundtrip_markdown_to_odoc_to_markdown()    L169
// fn convert_document_metadata()                 L188
// ---------------------------------------------------

fn opts() -> ConvertOptions {
    ConvertOptions {
        document_id: Some("urn:odoc:test/convert".to_string()),
        title: Some("Test".to_string()),
        created_at: Some("2026-03-17T00:00:00Z".to_string()),
        ..Default::default()
    }
}

#[test]
fn convert_simple_paragraph() {
    let doc = markdown::markdown("Hello world", &opts()).unwrap();
    assert_eq!(doc.content.len(), 1);
    assert_eq!(doc.content[0].kind(), "paragraph");
}

#[test]
fn convert_heading() {
    let doc = markdown::markdown("# Title\n\nBody text", &opts()).unwrap();
    assert_eq!(doc.content.len(), 2);
    assert_eq!(doc.content[0].kind(), "heading");
    if let odoc::Block::Heading { level, .. } = &doc.content[0] {
        assert_eq!(*level, 1);
    }
    assert_eq!(doc.content[1].kind(), "paragraph");
}

#[test]
fn convert_code_block() {
    let input = "```rust\nfn main() {}\n```";
    let doc = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc.content.len(), 1);
    if let odoc::Block::Code { language, text, .. } = &doc.content[0] {
        assert_eq!(language.as_deref(), Some("rust"));
        assert_eq!(text, "fn main() {}");
    } else {
        panic!("expected code block");
    }
}

#[test]
fn convert_list() {
    let input = "- item one\n- item two";
    let doc = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc.content.len(), 1);
    if let odoc::Block::List { ordered, items, .. } = &doc.content[0] {
        assert!(!ordered);
        assert_eq!(items.len(), 2);
    } else {
        panic!("expected list");
    }
}

#[test]
fn convert_ordered_list() {
    let input = "1. first\n2. second";
    let doc = markdown::markdown(input, &opts()).unwrap();
    if let odoc::Block::List { ordered, items, .. } = &doc.content[0] {
        assert!(ordered);
        assert_eq!(items.len(), 2);
    } else {
        panic!("expected ordered list");
    }
}

#[test]
fn convert_blockquote() {
    let input = "> quoted text";
    let doc = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc.content.len(), 1);
    assert_eq!(doc.content[0].kind(), "blockquote");
}

#[test]
fn convert_divider() {
    let input = "before\n\n---\n\nafter";
    let doc = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc.content.len(), 3);
    assert_eq!(doc.content[1].kind(), "divider");
}

#[test]
fn convert_table() {
    let input = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc.content.len(), 1);
    if let odoc::Block::Table { columns, rows, .. } = &doc.content[0] {
        assert_eq!(columns, &["A", "B"]);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], &["1", "2"]);
    } else {
        panic!("expected table");
    }
}

#[test]
fn convert_inline_marks() {
    let input = "**bold** and *italic* and `code` and ~~struck~~";
    let doc = markdown::markdown(input, &opts()).unwrap();
    if let odoc::Block::Paragraph { content, .. } = &doc.content[0] {
        // Check bold.
        if let odoc::InlineNode::Text { text, marks, .. } = &content[0] {
            assert_eq!(text, "bold");
            let marks = marks.as_ref().unwrap();
            assert!(marks.iter().any(|m| matches!(m, odoc::Mark::Strong)));
        }
    } else {
        panic!("expected paragraph");
    }
}

#[test]
fn convert_link() {
    let input = "[click here](https://example.com)";
    let doc = markdown::markdown(input, &opts()).unwrap();
    if let odoc::Block::Paragraph { content, .. } = &doc.content[0] {
        if let odoc::InlineNode::Text { marks, .. } = &content[0] {
            let marks = marks.as_ref().unwrap();
            assert!(marks.iter().any(|m| matches!(m, odoc::Mark::Link { .. })));
        }
    }
}

#[test]
fn convert_hard_break() {
    let input = "line one  \nline two";
    let doc = markdown::markdown(input, &opts()).unwrap();
    if let odoc::Block::Paragraph { content, .. } = &doc.content[0] {
        assert!(content.iter().any(|n| matches!(n, odoc::InlineNode::HardBreak { .. })));
    }
}

#[test]
fn sequential_ids_are_stable() {
    let input = "# Heading\n\nParagraph\n\n## Another heading";
    let doc1 = markdown::markdown(input, &opts()).unwrap();
    let doc2 = markdown::markdown(input, &opts()).unwrap();
    assert_eq!(doc1.content[0].id(), doc2.content[0].id());
    assert_eq!(doc1.content[1].id(), doc2.content[1].id());
    assert_eq!(doc1.content[2].id(), doc2.content[2].id());
}

#[test]
fn roundtrip_markdown_to_odoc_to_markdown() {
    let input = "# Hello\n\nWorld with **bold** text.\n\n- item one\n- item two\n";
    let doc = markdown::markdown(input, &opts()).unwrap();
    let rendered = odoc::render::markdown::markdown(&doc).unwrap();
    // Re-convert the rendered markdown and compare structure.
    let doc2 = markdown::markdown(&rendered, &opts()).unwrap();
    assert_eq!(
        doc.content.len(),
        doc2.content.len(),
        "content length mismatch.\noriginal kinds: {:?}\nroundtrip kinds: {:?}\nrendered:\n{rendered}",
        doc.content.iter().map(|b| b.kind()).collect::<Vec<_>>(),
        doc2.content.iter().map(|b| b.kind()).collect::<Vec<_>>(),
    );
    for (a, b) in doc.content.iter().zip(doc2.content.iter()) {
        assert_eq!(a.kind(), b.kind());
    }
}

#[test]
fn convert_document_metadata() {
    let opts = ConvertOptions {
        document_id: Some("urn:odoc:my-doc".to_string()),
        title: Some("My Doc".to_string()),
        created_at: Some("2026-03-17T00:00:00Z".to_string()),
        producer: Some("test".to_string()),
        language: Some("en".to_string()),
        ..Default::default()
    };
    let doc = markdown::markdown("content", &opts).unwrap();
    assert_eq!(doc.id, "urn:odoc:my-doc");
    assert_eq!(doc.meta.title, "My Doc");
    assert_eq!(doc.meta.producer.as_deref(), Some("test"));
    assert_eq!(doc.meta.language.as_deref(), Some("en"));
    assert_eq!(doc.odoc, "1.0.0");
}
