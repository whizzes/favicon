use std::borrow::Cow;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Error, Result};
use image::imageops::FilterType;
use image::{DynamicImage, ImageOutputFormat};

pub struct Preset {
    pub(crate) name: Cow<'static, str>,
    pub(crate) format: ImageOutputFormat,
    pub(crate) height: u32,
    pub(crate) width: u32,
}

impl Preset {
    pub const fn new_static(
        name: &'static str,
        format: ImageOutputFormat,
        height: u32,
        width: u32,
    ) -> Self {
        Self {
            name: Cow::Borrowed(name),
            format,
            height,
            width,
        }
    }
}

pub struct Favicon {
    pub(crate) image: DynamicImage,
    pub(crate) out: PathBuf,
    pub(crate) job_queue: Vec<Preset>,
}

impl Favicon {
    pub fn new(file: PathBuf, out_dir: PathBuf) -> Result<Self> {
        let image = image::open(file).unwrap();

        if image.width() != image.height() {
            return Err(Error::msg("The image is not square."));
        }

        Ok(Self {
            image,
            out: out_dir,
            job_queue: Vec::with_capacity(1),
        })
    }

    pub fn queue(&mut self, preset: Preset) {
        self.job_queue.push(preset);
    }

    pub fn process(&self) -> Result<()> {
        for preset in self.job_queue.iter() {
            let mut filename = self.out.clone();
            filename.push(PathBuf::from_str(preset.name.as_ref()).unwrap());

            let mut output = File::create(filename).unwrap();
            let resized = self
                .image
                .resize(preset.width, preset.height, FilterType::Triangle);

            resized
                .write_to(&mut output, ImageOutputFormat::Png)
                .unwrap();
        }

        Ok(())
    }
}
