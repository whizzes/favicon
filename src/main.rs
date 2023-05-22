mod image;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::image::Favicon;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'f', long)]
    file: PathBuf,

    /// Name of the person to greet
    #[arg(short = 'o', long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let favicon = Favicon::new(args.file, args.output)?;

    favicon.process()?;

    Ok(())
}
