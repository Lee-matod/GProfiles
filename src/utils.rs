use image::{DynamicImage, ImageReader};
use rfd::FileDialog;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use slint::{Image, SharedPixelBuffer};
use std::path;

pub const APPLICATION_NAME_DESKTOP: &str = "APPLICATION_NAME_DESKTOP";
pub const PROFILE_NAME_DEFAULT: &str = "PROFILE_NAME_DEFAULT";
pub const BROKEN_IMAGE_ICON: &[u8; 4275] = include_bytes!("../assets/broken_image.png");
pub const DESKTOP_ICON: &[u8; 3251] = include_bytes!("../assets/desktop.png");

pub fn parse_json<T: Serialize + for<'de> Deserialize<'de>>(file: &path::Path) -> T {
    let data: String;
    if file.extension() == Some("json".as_ref()) {
        data = std::fs::read_to_string(file).expect("Failed to read JSON file.");
    } else if file.extension() == Some("db".as_ref()) {
        let conn = Connection::open(file).expect("Failed to open database file.");
        let mut stmt = conn.prepare("SELECT file FROM data;").unwrap();
        let row: Vec<u8> = stmt.query_row([], |row| Ok(row.get(0)?)).unwrap();
        drop(stmt);
        conn.close().unwrap();
        data = String::from_utf8(row).expect("Failed to convert database data to string.");
    } else {
        // TODO: better error handling
        panic!("Unsupported file format for JSON parsing.");
    }

    serde_json::from_str::<T>(&data).expect("Failed to parse JSON data.")
}

pub fn file_picker(name: &str, ext: &[&str], dir: Option<&path::Path>) -> Option<path::PathBuf> {
    let mut dialog = FileDialog::new().add_filter(name, ext);
    if let Some(d) = dir {
        dialog = dialog.set_directory(d);
    }
    dialog.pick_file()
}

pub trait Cast<T> {
    fn using(value: T) -> Self;
}

impl Cast<DynamicImage> for Image {
    fn using(value: DynamicImage) -> Self {
        let buffer = value.to_rgba8();
        Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
            buffer.as_raw(),
            buffer.width(),
            buffer.height(),
        ))
    }
}

impl Cast<&path::Path> for Image {
    fn using(value: &path::Path) -> Self {
        if !value.exists() {
            Image::using(image::load_from_memory(BROKEN_IMAGE_ICON).unwrap())
        } else {
            let reader = ImageReader::open(value)
                .unwrap()
                .with_guessed_format()
                .unwrap();
            Image::using(reader.decode().unwrap())
        }
    }
}

impl Cast<path::PathBuf> for Image {
    fn using(value: path::PathBuf) -> Self {
        Image::using(value.as_path())
    }
}
