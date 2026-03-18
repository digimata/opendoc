use std::io;
use std::path::Path;

use anyhow::{Context, Result};

use crate::io::{read_document, read_input};
use crate::{ConvertFormat, RenderFormat};

// -----------------------
// projects/opendoc/crates/cli/src/cmd.rs
//
// pub fn convert()    L17
// pub fn render()     L63
// -----------------------

/// Convert an input file (markdown) to an `.odoc` JSON document.
pub fn convert(
    input: &str,
    from: &ConvertFormat,
    output: Option<&Path>,
    title: Option<String>,
    id: Option<String>,
    pretty: bool,
) -> Result<()> {
    let text = read_input(input)?;

    let options = odoc::convert::ConvertOptions {
        document_id: id,
        title,
        ..Default::default()
    };

    let doc = match from {
        ConvertFormat::Md => odoc::convert::markdown::markdown(&text, &options)
            .context("failed to convert markdown")?,
    };

    match output {
        Some(path) => {
            if pretty {
                odoc::io::to_path_pretty(path, &doc)
            } else {
                odoc::io::to_path(path, &doc)
            }
            .with_context(|| format!("failed to write {}", path.display()))?;
        }
        None => {
            let stdout = io::stdout().lock();
            if pretty {
                odoc::io::to_writer_pretty(stdout, &doc)
            } else {
                odoc::io::to_writer(stdout, &doc)
            }
            .context("failed to write to stdout")?;
            println!();
        }
    }

    Ok(())
}

/// Render an `.odoc` document to an output format (markdown or JSON).
pub fn render(input: &str, format: &RenderFormat, output: Option<&Path>) -> Result<()> {
    let doc = read_document(input)?;

    let rendered = match format {
        RenderFormat::Md => {
            odoc::render::markdown::markdown(&doc).context("failed to render markdown")?
        }
        RenderFormat::Json => {
            odoc::io::to_string_pretty(&doc).context("failed to serialize document")?
        }
    };

    match output {
        Some(path) => {
            std::fs::write(path, &rendered)
                .with_context(|| format!("failed to write {}", path.display()))?;
        }
        None => {
            print!("{rendered}");
        }
    }

    Ok(())
}
