use image::ImageOutputFormat;

use super::Preset;

pub const APPLE_TOUCH_ICON_57: Preset =
    Preset::new_static("apple_touch_icon-57.png", ImageOutputFormat::Png, 57, 57);
pub const APPLE_TOUCH_ICON_60: Preset =
    Preset::new_static("apple_touch_icon-60.png", ImageOutputFormat::Png, 60, 60);
pub const APPLE_TOUCH_ICON_72: Preset =
    Preset::new_static("apple_touch_icon-72.png", ImageOutputFormat::Png, 70, 70);
pub const APPLE_TOUCH_ICON_76: Preset =
    Preset::new_static("apple_touch_icon-76.png", ImageOutputFormat::Png, 76, 76);
pub const APPLE_TOUCH_ICON_114: Preset =
    Preset::new_static("apple_touch_icon-114.png", ImageOutputFormat::Png, 114, 114);
pub const APPLE_TOUCH_ICON_120: Preset =
    Preset::new_static("apple_touch_icon-120.png", ImageOutputFormat::Png, 120, 120);
pub const APPLE_TOUCH_ICON_144: Preset =
    Preset::new_static("apple_touch_icon-144.png", ImageOutputFormat::Png, 144, 144);
pub const APPLE_TOUCH_ICON_152: Preset =
    Preset::new_static("apple_touch_icon-152.png", ImageOutputFormat::Png, 152, 152);

pub const FAVICON: Preset = Preset::new_static("favicon.ico", ImageOutputFormat::Ico, 64, 64);
pub const FAVICON_16: Preset = Preset::new_static("favicon-16.png", ImageOutputFormat::Png, 16, 16);
pub const FAVICON_32: Preset = Preset::new_static("favicon-32.png", ImageOutputFormat::Png, 32, 32);
pub const FAVICON_96: Preset = Preset::new_static("favicon-96.png", ImageOutputFormat::Png, 96, 96);
pub const FAVICON_128: Preset =
    Preset::new_static("favicon-128.png", ImageOutputFormat::Png, 128, 128);
pub const FAVICON_196: Preset =
    Preset::new_static("favicon-32.png", ImageOutputFormat::Png, 196, 196);

pub const MS_TILE_70: Preset = Preset::new_static("mstile-70.png", ImageOutputFormat::Png, 70, 70);
pub const MS_TILE_144: Preset =
    Preset::new_static("mstile-144.png", ImageOutputFormat::Png, 144, 144);
pub const MS_TILE_150: Preset =
    Preset::new_static("mstile-150.png", ImageOutputFormat::Png, 150, 150);
pub const MS_TILE_310X150: Preset =
    Preset::new_static("mstile-310x150.png", ImageOutputFormat::Png, 310, 150);
pub const MS_TILE_310: Preset =
    Preset::new_static("mstile-310.png", ImageOutputFormat::Png, 310, 310);
