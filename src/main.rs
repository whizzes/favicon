mod image;
mod preset;

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::image::Favicon;

#[derive(Clone, Parser, Debug)]
pub enum Set {
    /// A set of Favicons that includes 16x16 and 32x32
    Small,
    /// Full set of Favicons that includes a favicon for each client
    Full,
}

impl From<OsString> for Set {
    fn from(s: OsString) -> Self {
        match s.to_str().unwrap() {
            "small" => Set::Small,
            "full" => Set::Full,
            _ => unreachable!(),
        }
    }
}

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

    #[arg(short = 's', long, default_value = "small")]
    set: Set,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut favicon = Favicon::new(args.file, args.output)?;

    match args.set {
        Set::Small => {
            use crate::preset::{FAVICON_16, FAVICON_32};

            favicon.queue(FAVICON_16);
            favicon.queue(FAVICON_32);
        }
        Set::Full => {
            use crate::preset::{
                APPLE_TOUCH_ICON_114, APPLE_TOUCH_ICON_120, APPLE_TOUCH_ICON_144,
                APPLE_TOUCH_ICON_152, APPLE_TOUCH_ICON_57, APPLE_TOUCH_ICON_60,
                APPLE_TOUCH_ICON_72, APPLE_TOUCH_ICON_76, FAVICON, FAVICON_128, FAVICON_16,
                FAVICON_196, FAVICON_96, MS_TILE_144, MS_TILE_150, MS_TILE_310, MS_TILE_310X150,
                MS_TILE_70,
            };

            // TODO: Provide a `queue_many` method
            favicon.queue(APPLE_TOUCH_ICON_114);
            favicon.queue(APPLE_TOUCH_ICON_120);
            favicon.queue(APPLE_TOUCH_ICON_144);
            favicon.queue(APPLE_TOUCH_ICON_152);
            favicon.queue(APPLE_TOUCH_ICON_57);
            favicon.queue(APPLE_TOUCH_ICON_60);
            favicon.queue(APPLE_TOUCH_ICON_72);
            favicon.queue(APPLE_TOUCH_ICON_76);
            favicon.queue(FAVICON);
            favicon.queue(FAVICON_16);
            favicon.queue(FAVICON_96);
            favicon.queue(FAVICON_128);
            favicon.queue(FAVICON_196);
            favicon.queue(MS_TILE_144);
            favicon.queue(MS_TILE_310);
            favicon.queue(MS_TILE_310X150);
            favicon.queue(MS_TILE_70);
            favicon.queue(MS_TILE_150);
        }
    }

    favicon.process()?;

    Ok(())
}
