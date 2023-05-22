mod image;

use std::path::PathBuf;

use clap::Parser;

use crate::image::resize;

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

fn main() {
    let args = Args::parse();

    resize(&args.file, &args.output);
}
