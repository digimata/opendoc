use std::io::Cursor;

// ----------------------------------
// projects/opendoc/crates/odoc/tests/io.rs
//
// fn from_str_roundtrip()        L16
// fn from_slice()                L26
// fn from_reader()               L33
// fn to_writer_roundtrip()       L41
// fn file_roundtrip()            L50
// fn from_str_invalid_json()     L62
// fn from_path_missing_file()    L68
// ----------------------------------

#[test]
fn from_str_roundtrip() {
    let json = include_str!("corpus/json/simple.json");
    let doc = odoc::io::from_str(json).unwrap();
    assert_eq!(doc.id, "urn:odoc:test/simple");
    let out = odoc::io::to_string_pretty(&doc).unwrap();
    let reparsed = odoc::io::from_str(&out).unwrap();
    assert_eq!(doc, reparsed);
}

#[test]
fn from_slice() {
    let json = include_bytes!("corpus/json/simple.json");
    let doc = odoc::io::from_slice(json).unwrap();
    assert_eq!(doc.id, "urn:odoc:test/simple");
}

#[test]
fn from_reader() {
    let json = include_str!("corpus/json/simple.json");
    let cursor = Cursor::new(json.as_bytes());
    let doc = odoc::io::from_reader(cursor).unwrap();
    assert_eq!(doc.id, "urn:odoc:test/simple");
}

#[test]
fn to_writer_roundtrip() {
    let doc = odoc::io::from_str(include_str!("corpus/json/full.json")).unwrap();
    let mut buf = Vec::new();
    odoc::io::to_writer(&mut buf, &doc).unwrap();
    let reparsed = odoc::io::from_slice(&buf).unwrap();
    assert_eq!(doc, reparsed);
}

#[test]
fn file_roundtrip() {
    let doc = odoc::io::from_str(include_str!("corpus/json/annotated.json")).unwrap();
    let dir = std::env::temp_dir().join("odoc_test_io");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("roundtrip.json");
    odoc::io::to_path_pretty(&path, &doc).unwrap();
    let loaded = odoc::io::from_path(&path).unwrap();
    assert_eq!(doc, loaded);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn from_str_invalid_json() {
    let err = odoc::io::from_str("not json").unwrap_err();
    assert!(matches!(err, odoc::io::ReadError::Json { .. }));
}

#[test]
fn from_path_missing_file() {
    let err = odoc::io::from_path("/nonexistent/path.json").unwrap_err();
    assert!(matches!(err, odoc::io::ReadError::Io { .. }));
}
