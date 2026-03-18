// ----------------------------------------
// projects/opendoc/crates/odoc/tests/query.rs
//
// fn index_simple()                    L14
// fn index_full_with_annotations()     L33
// fn block_helpers()                   L53
// fn inline_helpers()                  L62
// fn document_annotations_helper()     L73
// fn target_node_ids()                 L84
// fn duplicate_node_id_detected()     L124
// ----------------------------------------

#[test]
fn index_simple() {
    let doc = odoc::io::from_str(include_str!("corpus/json/simple.json")).unwrap();
    let idx = doc.index().unwrap();

    // Block lookup.
    assert!(idx.block("h1").is_some());
    assert!(idx.block("p1").is_some());
    assert!(idx.block("c1").is_some());
    assert!(idx.block("d1").is_some());

    // Inline lookup.
    assert!(idx.inline("t1").is_some());
    assert!(idx.inline("t2").is_some());

    // Non-existent.
    assert!(idx.node("zzz").is_none());
}

#[test]
fn index_full_with_annotations() {
    let doc = odoc::io::from_str(include_str!("corpus/json/full.json")).unwrap();
    let idx = doc.index().unwrap();

    // Annotations.
    assert!(idx.annotation("a1").is_some());
    assert!(idx.annotation("a2").is_some());
    let targeting_p1 = idx.annotations_targeting("p1");
    assert!(!targeting_p1.is_empty());

    // Provenance.
    assert!(idx.provenance("pr1").is_some());
    assert!(idx.provenance("pr2").is_some());

    // List items.
    assert!(idx.list_item("li1").is_some());
    assert!(idx.list_item("li2").is_some());
}

#[test]
fn block_helpers() {
    let doc = odoc::io::from_str(include_str!("corpus/json/simple.json")).unwrap();
    let h = &doc.content[0];
    assert_eq!(h.id(), "h1");
    assert_eq!(h.kind(), "heading");
    assert!(h.prov_refs().is_empty());
}

#[test]
fn inline_helpers() {
    let doc = odoc::io::from_str(include_str!("corpus/json/simple.json")).unwrap();
    if let odoc::Block::Paragraph { content, .. } = &doc.content[1] {
        assert_eq!(content[0].id(), "t2");
        assert_eq!(content[0].kind(), "text");
    } else {
        panic!("expected paragraph");
    }
}

#[test]
fn document_annotations_helper() {
    let doc = odoc::io::from_str(include_str!("corpus/json/full.json")).unwrap();
    assert_eq!(doc.annotations().len(), 2);
    assert_eq!(doc.provenance().len(), 2);

    let simple = odoc::io::from_str(include_str!("corpus/json/simple.json")).unwrap();
    assert!(simple.annotations().is_empty());
    assert!(simple.provenance().is_empty());
}

#[test]
fn target_node_ids() {
    use odoc::{BoundaryPoint, Target};

    let node = Target::Node {
        node: "p1".into(),
    };
    assert_eq!(node.node_ids(), vec!["p1"]);

    let range = Target::TextRange {
        start: BoundaryPoint {
            node: "t1".into(),
            offset: 0,
        },
        end: BoundaryPoint {
            node: "t2".into(),
            offset: 5,
        },
    };
    assert_eq!(range.node_ids(), vec!["t1", "t2"]);

    let same_node_range = Target::TextRange {
        start: BoundaryPoint {
            node: "t1".into(),
            offset: 0,
        },
        end: BoundaryPoint {
            node: "t1".into(),
            offset: 5,
        },
    };
    assert_eq!(same_node_range.node_ids(), vec!["t1"]);

    let node_range = Target::NodeRange {
        start: "p1".into(),
        end: "p3".into(),
    };
    assert_eq!(node_range.node_ids(), vec!["p1", "p3"]);
}

#[test]
fn duplicate_node_id_detected() {
    // Build a document with duplicate block IDs.
    let json = r#"{
        "$schema": "https://opendoc.dev/schema/odoc/1.0.0.json",
        "odoc": "1.0.0",
        "id": "test",
        "meta": {"title": "test", "created_at": "2026-01-01"},
        "content": [
            {"id": "dup", "type": "divider"},
            {"id": "dup", "type": "divider"}
        ]
    }"#;
    let doc: odoc::Document = serde_json::from_str(json).unwrap();
    let err = doc.index().unwrap_err();
    assert!(matches!(err, odoc::query::IndexError::DuplicateNodeId { .. }));
}
