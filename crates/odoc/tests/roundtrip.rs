use odoc::Document;

// ---------------------------------------
// projects/opendoc/crates/odoc/tests/roundtrip.rs
//
// fn roundtrip()                      L18
// fn roundtrip_simple()               L27
// fn roundtrip_annotated()            L32
// fn roundtrip_full()                 L37
// fn all_mark_variants()              L42
// fn all_target_variants()            L63
// fn all_provenance_kinds()           L79
// fn all_block_types_deserialize()    L92
// ---------------------------------------

/// Deserialize a fixture, re-serialize, and compare against the original JSON
/// using Value equality (order-insensitive for objects).
fn roundtrip(json_str: &str) {
    let original: serde_json::Value = serde_json::from_str(json_str).expect("fixture is valid JSON");
    let doc: Document = serde_json::from_str(json_str).expect("fixture deserializes to Document");
    let reserialized: serde_json::Value =
        serde_json::to_value(&doc).expect("Document serializes to Value");
    assert_eq!(original, reserialized);
}

#[test]
fn roundtrip_simple() {
    roundtrip(include_str!("corpus/json/simple.json"));
}

#[test]
fn roundtrip_annotated() {
    roundtrip(include_str!("corpus/json/annotated.json"));
}

#[test]
fn roundtrip_full() {
    roundtrip(include_str!("corpus/json/full.json"));
}

#[test]
fn all_mark_variants() {
    let marks_json = r#"[
        {"type": "strong"},
        {"type": "em"},
        {"type": "code"},
        {"type": "strikethrough"},
        {"type": "underline"},
        {"type": "sup"},
        {"type": "sub"},
        {"type": "link", "href": "https://example.com", "title": "Example"},
        {"type": "highlight", "color": "yellow"},
        {"type": "footnote", "note": "A footnote."}
    ]"#;
    let marks: Vec<odoc::Mark> = serde_json::from_str(marks_json).expect("marks deserialize");
    assert_eq!(marks.len(), 10);
    let reserialized: serde_json::Value = serde_json::to_value(&marks).unwrap();
    let original: serde_json::Value = serde_json::from_str(marks_json).unwrap();
    assert_eq!(original, reserialized);
}

#[test]
fn all_target_variants() {
    let node_json = r#"{"kind": "node", "node": "p1"}"#;
    let target: odoc::Target = serde_json::from_str(node_json).unwrap();
    assert!(matches!(target, odoc::Target::Node { .. }));

    let text_range_json =
        r#"{"kind": "text_range", "start": {"node": "t1", "offset": 0}, "end": {"node": "t1", "offset": 5}}"#;
    let target: odoc::Target = serde_json::from_str(text_range_json).unwrap();
    assert!(matches!(target, odoc::Target::TextRange { .. }));

    let node_range_json = r#"{"kind": "node_range", "start": "p1", "end": "p3"}"#;
    let target: odoc::Target = serde_json::from_str(node_range_json).unwrap();
    assert!(matches!(target, odoc::Target::NodeRange { .. }));
}

#[test]
fn all_provenance_kinds() {
    for kind in ["source", "assertion", "derived"] {
        let json = format!(
            r#"{{"id": "pr1", "kind": "{kind}", "body": {{"method": "test"}}}}"#
        );
        let record: odoc::ProvenanceRecord = serde_json::from_str(&json).unwrap();
        let reserialized: serde_json::Value = serde_json::to_value(&record).unwrap();
        let original: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(original, reserialized);
    }
}

#[test]
fn all_block_types_deserialize() {
    let blocks_json = r#"[
        {"id": "p1", "type": "paragraph", "content": [{"id": "t1", "type": "text", "text": "hello"}]},
        {"id": "h1", "type": "heading", "level": 2, "content": [{"id": "t2", "type": "text", "text": "title"}]},
        {"id": "l1", "type": "list", "ordered": false, "items": [{"id": "li1", "content": [{"id": "p-li1", "type": "paragraph", "content": [{"id": "t3", "type": "text", "text": "item"}]}]}]},
        {"id": "q1", "type": "blockquote", "content": [{"id": "p-q1", "type": "paragraph", "content": [{"id": "t4", "type": "text", "text": "quote"}]}]},
        {"id": "c1", "type": "code", "text": "println!()"},
        {"id": "tbl1", "type": "table", "columns": ["a"], "rows": [["1"]]},
        {"id": "img1", "type": "image", "src": "https://example.com/img.png"},
        {"id": "d1", "type": "divider"},
        {"id": "emb1", "type": "embed", "src": "urn:osheet:test"}
    ]"#;
    let blocks: Vec<odoc::Block> = serde_json::from_str(blocks_json).expect("all blocks deserialize");
    assert_eq!(blocks.len(), 9);
}
