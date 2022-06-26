use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hasher;
use std::path::{Path, PathBuf};

pub const APPLE_TOUCH_ICON_57: Preset<'static> =
    Preset::new("apple_touch_icon-57.png", Format::Png, 57, 57);
pub const APPLE_TOUCH_ICON_60: Preset<'static> =
    Preset::new("apple_touch_icon-60.png", Format::Png, 60, 60);
pub const APPLE_TOUCH_ICON_72: Preset<'static> =
    Preset::new("apple_touch_icon-72.png", Format::Png, 70, 70);
pub const APPLE_TOUCH_ICON_76: Preset<'static> =
    Preset::new("apple_touch_icon-76.png", Format::Png, 76, 76);
pub const APPLE_TOUCH_ICON_114: Preset<'static> =
    Preset::new("apple_touch_icon-114.png", Format::Png, 114, 114);
pub const APPLE_TOUCH_ICON_120: Preset<'static> =
    Preset::new("apple_touch_icon-120.png", Format::Png, 120, 120);
pub const APPLE_TOUCH_ICON_144: Preset<'static> =
    Preset::new("apple_touch_icon-144.png", Format::Png, 144, 144);
pub const APPLE_TOUCH_ICON_152: Preset<'static> =
    Preset::new("apple_touch_icon-152.png", Format::Png, 152, 152);

pub const FAVICON: Preset<'static> = Preset::new("favicon.ico", Format::Ico, 64, 64);
pub const FAVICON_16: Preset<'static> = Preset::new("favicon-16.png", Format::Png, 16, 16);
pub const FAVICON_32: Preset<'static> = Preset::new("favicon-32.png", Format::Png, 32, 32);
pub const FAVICON_96: Preset<'static> = Preset::new("favicon-96.png", Format::Png, 96, 96);
pub const FAVICON_128: Preset<'static> = Preset::new("favicon-128.png", Format::Png, 128, 128);
pub const FAVICON_196: Preset<'static> = Preset::new("favicon-32.png", Format::Png, 196, 196);

pub const MS_TILE_70: Preset<'static> = Preset::new("mstile-70.png", Format::Png, 70, 70);
pub const MS_TILE_144: Preset<'static> = Preset::new("mstile-144.png", Format::Png, 144, 144);
pub const MS_TILE_150: Preset<'static> = Preset::new("mstile-150.png", Format::Png, 150, 150);
pub const MS_TILE_310X150: Preset<'static> =
    Preset::new("mstile-310x150.png", Format::Png, 310, 150);
pub const MS_TILE_310: Preset<'static> = Preset::new("mstile-310.png", Format::Png, 310, 310);

pub enum Format {
    Ico,
    Png,
}

pub struct Preset<'a> {
    pub(crate) name: &'a str,
    pub(crate) format: Format,
    pub(crate) height: u32,
    pub(crate) width: u32,
}

impl<'a> Preset<'a> {
    pub const fn new(name: &'a str, format: Format, height: u32, width: u32) -> Self {
        Self {
            name,
            format,
            height,
            width,
        }
    }
}

pub struct Favicon<'a, P: AsRef<Path> + Copy> {
    pub(crate) image: DynamicImage,
    pub(crate) out_dir: P,
    pub(crate) presets: Vec<Preset<'a>>,
}

impl<'a, P: AsRef<Path> + Copy> Favicon<'a, P> {
    pub fn new(file: P, out_dir: P, presets: Vec<Preset<'a>>) -> Self
    where
        P: AsRef<Path>,
    {
        let image = image::open(file).unwrap();

        if image.width() != image.height() {
            println!("The image is not square. This will result in rectangle like image.");
        }

        Favicon::create_output_dir(out_dir.clone());

        Self {
            image,
            out_dir,
            presets,
        }
    }

    pub fn empty(file: P, out_dir: P) -> Self {
        let image = image::open(file).unwrap();

        if image.width() != image.height() {
            println!("The image is not square. This will result in rectangle like image.");
        }

        Favicon::create_output_dir(out_dir);

        Self {
            image,
            out_dir,
            presets: Vec::default(),
        }
    }

    pub fn exec(&self) {
        let images = self.resize_all();

        for (preset_name, image) in images.iter() {
            let output_path = self.output_path(preset_name);
            let mut output = File::create(output_path).unwrap();

            image.write_to(&mut output, ImageFormat::Png).unwrap();
        }
    }

    pub fn add_preset(&mut self, preset: Preset<'a>) {
        self.presets.push(preset);
    }

    pub fn resize_all(&self) -> HashMap<String, DynamicImage> {
        let mut images: HashMap<String, DynamicImage> = HashMap::default();

        for preset in self.presets.iter() {
            let image = self.resize(preset);

            images.insert(preset.name.to_string(), image);
        }

        images
    }

    fn output_path(&self, filename: &str) -> PathBuf {
        let mut path = PathBuf::new();

        path.push(self.out_dir);
        path.push(filename);

        path
    }

    fn create_output_dir(out_dir: P) {
        fs::create_dir_all(out_dir).unwrap();
    }

    fn resize(&self, preset: &'a Preset<'a>) -> DynamicImage {
        self
            .image
            .resize(preset.width, preset.height, FilterType::Lanczos3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resizes_image_for_multiple_presets() {
        let mut favicon = Favicon::empty("fixtures/rust.png", "tmp");

        favicon.add_preset(APPLE_TOUCH_ICON_57);
        favicon.add_preset(APPLE_TOUCH_ICON_152);
        favicon.add_preset(APPLE_TOUCH_ICON_144);

        favicon.exec();
    }
}
