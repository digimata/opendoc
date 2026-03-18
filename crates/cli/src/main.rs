mod cmd;
mod io;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

// -----------------------------
// projects/opendoc/crates/cli/src/main.rs
//
// mod cmd                    L1
// mod io                     L2
// struct Cli                L26
// enum Command              L32
// pub enum ConvertFormat    L69
// pub enum RenderFormat     L75
// fn main()                 L82
// -----------------------------

#[derive(Debug, Parser)]
#[command(
    name = "odoc",
    version,
    about = "OpenDoc CLI — convert and render .odoc documents"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Convert an input file to .odoc format.
    Convert {
        /// Input file path (use "-" or omit for stdin).
        #[arg(default_value = "-")]
        input: String,
        /// Input format.
        #[arg(short = 'f', long, default_value = "md")]
        from: ConvertFormat,
        /// Output file path (default: stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Document title (metadata).
        #[arg(long)]
        title: Option<String>,
        /// Document ID (default: auto-generated).
        #[arg(long)]
        id: Option<String>,
        /// Pretty-print the JSON output.
        #[arg(long)]
        pretty: bool,
    },
    /// Render an .odoc file to an output format.
    Render {
        /// Input .odoc file path (use "-" or omit for stdin).
        #[arg(default_value = "-")]
        input: String,
        /// Output format.
        #[arg(short = 'f', long, default_value = "md")]
        format: RenderFormat,
        /// Output file path (default: stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ConvertFormat {
    /// Markdown
    Md,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RenderFormat {
    /// Markdown
    Md,
    /// JSON (re-serialize / pretty-print)
    Json,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Convert {
            input,
            from,
            output,
            title,
            id,
            pretty,
        } => cmd::convert(&input, &from, output.as_deref(), title, id, pretty),
        Command::Render {
            input,
            format,
            output,
        } => cmd::render(&input, &format, output.as_deref()),
    };

    if let Err(error) = result {
        eprintln!("odoc: {error:#}");
        std::process::exit(1);
    }
}
