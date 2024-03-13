use std::path;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat, RgbaImage};

pub struct Image {
    reader: DynamicImage,
}

impl Image {
    pub fn from_rgba(rgba: RgbaImage) -> Image {
        let reader = DynamicImage::from(rgba);
        Image { reader }
    }

    pub fn from_path(path: &path::Path) -> Image {
        let reader = ImageReader::open(path).unwrap().decode().unwrap();
        Image { reader }
    }

    pub fn save_to_cache(&self, id: String) -> path::PathBuf {
        let localappdata_env = option_env!("LOCALAPPDATA").expect("no %localappdata% directory");

        let localappdata = path::Path::new(localappdata_env);
        let ghub_folder = localappdata.join("LGHUB");
        if !ghub_folder.exists() {
            panic!("LGHUB folder not in %localappdata%");
        }
        let icon_cache = ghub_folder.join("icon_cache");
        if !icon_cache.exists() {
            panic!("icon_cache not in LGHUB");
        }
        let filepath = icon_cache.join(id + ".bmp");
        let reader = self.reader.resize(256, 256, FilterType::CatmullRom);
        reader
            .save_with_format(&filepath, ImageFormat::Bmp)
            .unwrap();
        filepath
    }

    pub fn load_from_cache(&self) -> RgbaImage {
        let reader = self.reader.resize(48, 48, FilterType::CatmullRom);
        reader.to_rgba8()
    }
}
