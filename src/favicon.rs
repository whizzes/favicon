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
            filename.push(PathBuf::from_str(preset.name)?);

            let mut output = File::create(filename)?;
            let resized = self
                .image
                .resize(preset.width, preset.height, FilterType::Triangle);

            resized.write_to(&mut output, preset.format.clone())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use image::ImageOutputFormat;

    use crate::favicon::Favicon;
    use crate::preset::Preset;

    #[test]
    fn stores_presets_in_job_queue() {
        let mut favicon = Favicon::new(PathBuf::from("fixtures/rust.png")).unwrap();

        favicon.queue(Preset::new(
            "favicon-16x16.png",
            ImageOutputFormat::Png,
            16,
            16,
        ));
        favicon.queue(Preset::new(
            "favicon-32x32.png",
            ImageOutputFormat::Png,
            32,
            32,
        ));

        assert_eq!(favicon.job_queue.len(), 2);
        assert_eq!(
            favicon.job_queue[0],
            Preset::new("favicon-16x16.png", ImageOutputFormat::Png, 16, 16,)
        );
        assert_eq!(
            favicon.job_queue[1],
            Preset::new("favicon-32x32.png", ImageOutputFormat::Png, 32, 32,)
        );
    }
}
