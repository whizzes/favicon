mod favicon;
mod preset;

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};

use crate::favicon::Favicon;

#[derive(Clone, Parser, Debug, Default)]
pub enum Set {
    /// A set of Favicons that includes 16x16 and 32x32
    #[default]
    Small,
    /// Full set of Favicons that includes a favicon for each client
    Full,
}

impl ValueEnum for Set {
    fn value_variants<'a>() -> &'a [Self] {
        &[Set::Small, Set::Full]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Small => {
                PossibleValue::new("small").help("A set of Favicons that includes 16x16 and 32x32")
            }
            Self::Full => PossibleValue::new("full")
                .help("Full set of Favicons that includes a favicon for each client"),
        })
    }
}

impl TryFrom<OsString> for Set {
    type Error = anyhow::Error;

    fn try_from(s: OsString) -> Result<Self> {
        use anyhow::Error;

        let Some(string) = s.to_str() else {
            return Err(Error::msg("String is not UTF-8."));
        };

        match string {
            "small" => Ok(Set::Small),
            "full" => Ok(Set::Full),
            _ => Err(Error::msg("Invalid set. Values are `small` or `full`.")),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to create favicons from
    #[arg(index = 1)]
    file: PathBuf,

    /// Output directory to put favicons
    #[arg(short = 'o', long)]
    output: PathBuf,

    /// Colllection of favicons to generate
    #[arg(short = 's', long)]
    set: Set,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut favicon = Favicon::new(args.file)?;

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

    favicon.process(args.output)?;

    Ok(())
}
