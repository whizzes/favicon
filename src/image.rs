use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Error, Result};
use image::imageops::FilterType;
use image::DynamicImage;

use crate::preset::Preset;

pub struct Favicon {
    pub(crate) image: DynamicImage,
    pub(crate) job_queue: Vec<Preset>,
}

impl Favicon {
    /// Creates a new instance of Favicon
    pub fn new(file: PathBuf) -> Result<Self> {
        let image = image::open(file)?;

        if image.width() != image.height() {
            return Err(Error::msg("The image is not square."));
        }

        Ok(Self {
            image,
            job_queue: Vec::with_capacity(1),
        })
    }

    pub fn queue(&mut self, preset: Preset) {
        self.job_queue.push(preset);
    }

    pub fn process(&self, out_dir: PathBuf) -> Result<()> {
        for preset in self.job_queue.iter() {
            let mut filename = out_dir.clone();
            filename.push(PathBuf::from_str(preset.name.as_ref())?);

            let mut output = File::create(filename)?;
            let resized = self
                .image
                .resize(preset.width, preset.height, FilterType::Triangle);

            resized.write_to(&mut output, preset.format.clone())?;
        }

        Ok(())
    }
}
