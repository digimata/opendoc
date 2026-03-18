// ----------------------------
// projects/opendoc/crates/odoc/src/render/mod.rs
//
// pub mod markdown         L14
// pub enum OutputFormat    L23
// pub trait Render         L29
//   fn render()            L33
// pub fn to_writer()       L40
// pub fn to_path()         L51
// pub enum RenderError     L65
// ----------------------------

#[cfg(feature = "render-markdown")]
pub mod markdown;

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::Document;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Markdown,
    Html,
    Text,
}

pub trait Render {
    fn format(&self) -> OutputFormat;
    fn render_into(&self, doc: &Document, out: &mut dyn fmt::Write) -> Result<(), RenderError>;

    fn render(&self, doc: &Document) -> Result<String, RenderError> {
        let mut buf = String::new();
        self.render_into(doc, &mut buf)?;
        Ok(buf)
    }
}

pub fn to_writer<R: Render + ?Sized, W: io::Write>(
    renderer: &R,
    doc: &Document,
    mut writer: W,
) -> Result<(), RenderError> {
    let text = renderer.render(doc)?;
    writer
        .write_all(text.as_bytes())
        .map_err(|source| RenderError::Io { path: None, source })
}

pub fn to_path<R: Render + ?Sized>(
    renderer: &R,
    doc: &Document,
    path: impl AsRef<Path>,
) -> Result<(), RenderError> {
    let path = path.as_ref();
    let text = renderer.render(doc)?;
    std::fs::write(path, text.as_bytes()).map_err(|source| RenderError::Io {
        path: Some(path.to_path_buf()),
        source,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("io error writing {path:?}: {source}")]
    Io {
        path: Option<PathBuf>,
        source: io::Error,
    },
    #[error("format error: {0}")]
    Fmt(#[from] fmt::Error),
}
