use std::io::{self, Read};

use anyhow::{Context, Result};

// -----------------------------
// projects/opendoc/crates/cli/src/io.rs
//
// pub fn read_input()       L13
// pub fn read_document()    L26
// -----------------------------

/// Read text content from a file path, or stdin if path is "-".
pub fn read_input(input: &str) -> Result<String> {
    if input == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .context("failed to read stdin")?;
        Ok(buf)
    } else {
        std::fs::read_to_string(input).with_context(|| format!("failed to read {input}"))
    }
}

/// Read and parse an `.odoc` document from a file path, or stdin if path is "-".
pub fn read_document(input: &str) -> Result<odoc::Document> {
    if input == "-" {
        let stdin = io::stdin().lock();
        odoc::io::from_reader(stdin).context("failed to parse .odoc from stdin")
    } else {
        odoc::io::from_path(input).with_context(|| format!("failed to read .odoc from {input}"))
    }
}
