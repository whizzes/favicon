use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use std::fs::{self, File};
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

    pub fn add_preset(&mut self, preset: Preset<'a>) {
        self.presets.push(preset);
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

    fn resize(&self, preset: &'a Preset<'a>) {
        let scaled = self
            .image
            .resize(preset.width, preset.height, FilterType::Lanczos3);
        let output_path = self.output_path(preset.name);
        let mut output = File::create(output_path).unwrap();

        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    }

    pub fn resize_all(&self) {
        for preset in self.presets.iter() {
            self.resize(preset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resizes_an_image() {
        let input_image = image::open("fixtures/rust.png").unwrap();
        let input_height = input_image.height();
        let input_width = input_image.width();

        let favicon = Favicon::empty("fixtures/rust.png", "tmp");
        favicon.resize(&APPLE_TOUCH_ICON_57);

        let output_filename = favicon.output_path(APPLE_TOUCH_ICON_57.name);
        let output_image = image::open(output_filename).unwrap();
        let output_height = output_image.height();
        let output_width = output_image.width();

        assert_ne!(input_height, output_height, "image height has changed");
        assert_ne!(input_width, output_width, "image width has changed");
        assert_eq!(
            output_width, APPLE_TOUCH_ICON_57.width,
            "image width is equals to preset's"
        );
    }

    #[test]
    fn resizes_image_for_multiple_presets() {
        let mut favicon = Favicon::empty("fixtures/rust.png", "tmp");

        favicon.add_preset(APPLE_TOUCH_ICON_57);
        favicon.add_preset(APPLE_TOUCH_ICON_152);
        favicon.add_preset(APPLE_TOUCH_ICON_144);

        favicon.resize_all();
    }
}
