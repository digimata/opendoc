use odoc::render::markdown;

// -------------------------------
// projects/opendoc/crates/odoc/tests/render.rs
//
// fn render_simple()          L13
// fn render_annotated()       L21
// fn render_full()            L29
// fn render_trait_format()    L37
// -------------------------------

#[test]
fn render_simple() {
    let doc = odoc::io::from_str(include_str!("corpus/json/simple.json")).unwrap();
    let rendered = markdown::markdown(&doc).unwrap();
    let expected = include_str!("corpus/markdown/simple.expected");
    assert_eq!(rendered, expected, "\n--- RENDERED ---\n{rendered}\n--- EXPECTED ---\n{expected}");
}

#[test]
fn render_annotated() {
    let doc = odoc::io::from_str(include_str!("corpus/json/annotated.json")).unwrap();
    let rendered = markdown::markdown(&doc).unwrap();
    let expected = include_str!("corpus/markdown/annotated.expected");
    assert_eq!(rendered, expected, "\n--- RENDERED ---\n{rendered}\n--- EXPECTED ---\n{expected}");
}

#[test]
fn render_full() {
    let doc = odoc::io::from_str(include_str!("corpus/json/full.json")).unwrap();
    let rendered = markdown::markdown(&doc).unwrap();
    let expected = include_str!("corpus/markdown/full.expected");
    assert_eq!(rendered, expected, "\n--- RENDERED ---\n{rendered}\n--- EXPECTED ---\n{expected}");
}

#[test]
fn render_trait_format() {
    use odoc::render::{OutputFormat, Render};
    let r = markdown::MarkdownRenderer;
    assert_eq!(r.format(), OutputFormat::Markdown);
}
