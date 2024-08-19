use std::fs;
use std::path;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat, RgbaImage};
use slint::SharedPixelBuffer;

use crate::extract::get_icon;
use crate::utils::MessageBox;
use crate::utils::{logitech_folder, safe_canonicalize, BROKEN_IMAGE_ICON};

pub struct Image {
    pub filename: String,
    reader: DynamicImage,
}

impl Image {
    pub fn icon_cache() -> path::PathBuf {
        let parent = logitech_folder();
        let icon_cache = parent.join("icon_cache");
        if !icon_cache.exists() {
            MessageBox::from("%LOCALAPPDATA%\\LGHUB\\icon_cache is not a directory.").error();
            panic!();
        }
        icon_cache
    }

    pub fn to_slint(&self, width: u32, height: u32) -> slint::Image {
        let reader = self
            .reader
            .resize(width, height, FilterType::CatmullRom)
            .to_rgba8();
        slint::Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
            reader.as_raw(),
            reader.width(),
            reader.height(),
        ))
    }

    pub fn with_filename(self, filename: String) -> Image {
        Image {
            filename,
            reader: self.reader,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) -> () {
        self.reader = self.reader.resize(width, height, FilterType::CatmullRom);
    }

    pub fn to_rgba(&self) -> RgbaImage {
        self.reader.to_rgba8()
    }

    pub fn save(&mut self) -> path::PathBuf {
        let icon_cache = Image::icon_cache();
        let fp = icon_cache.join(self.filename.clone() + ".bmp");
        self.resize(256, 256);
        self.reader.save_with_format(&fp, ImageFormat::Bmp).unwrap();
        fp
    }

    pub fn delete(&self) -> () {
        let icon_cache = Image::icon_cache();
        let fp = icon_cache.join(self.filename.clone() + ".bmp");
        if !fp.exists() {
            return;
        }
        fs::remove_file(fp).unwrap_or(());
    }
}

impl From<DynamicImage> for Image {
    fn from(value: DynamicImage) -> Self {
        Image {
            filename: String::new(),
            reader: value,
        }
    }
}

impl From<RgbaImage> for Image {
    fn from(value: RgbaImage) -> Self {
        Image {
            filename: String::new(),
            reader: DynamicImage::from(value),
        }
    }
}

impl From<&path::Path> for Image {
    fn from(value: &path::Path) -> Self {
        if !value.exists() {
            Image {
                filename: String::new(),
                reader: image::load_from_memory(BROKEN_IMAGE_ICON).unwrap(),
            }
        } else if value.extension().unwrap() == "exe" {
            Image::from(unsafe { get_icon(&safe_canonicalize(value)).unwrap() })
                .with_filename(value.file_stem().unwrap().to_string_lossy().to_string())
        } else {
            let reader = ImageReader::open(value)
                .unwrap()
                .with_guessed_format()
                .unwrap();
            Image {
                filename: value.file_stem().unwrap().to_string_lossy().to_string(),
                reader: reader.decode().unwrap(),
            }
        }
    }
}

impl From<path::PathBuf> for Image {
    fn from(value: path::PathBuf) -> Self {
        Image::from(value.as_path())
    }
}

impl From<String> for Image {
    fn from(value: String) -> Self {
        if value.starts_with("http") {
            let banner_name = value.split("/").last().unwrap();
            let depots = path::Path::new("C:\\ProgramData\\LGHUB\\depots");
            let parent = depots.read_dir().unwrap().last().unwrap().unwrap();
            Image::from(parent.path().join("core_apps\\images").join(banner_name))
        } else {
            let icon_cache = Image::icon_cache();
            let fp = icon_cache.join(value + ".bmp");
            Image::from(fp)
        }
    }
}
