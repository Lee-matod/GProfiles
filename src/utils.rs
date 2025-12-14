use image::{DynamicImage, ImageReader};
use rfd::FileDialog;
use rusqlite::{Connection, types::FromSql};
use serde::{Deserialize, Serialize};
use slint::{Image, SharedPixelBuffer};
use std::{io, path};

pub const APPLICATION_NAME_DESKTOP: &str = "APPLICATION_NAME_DESKTOP";
pub const PROFILE_NAME_DEFAULT: &str = "PROFILE_NAME_DEFAULT";
pub const BROKEN_IMAGE_ICON: &[u8; 4275] = include_bytes!("../assets/broken_image.png");
pub const DESKTOP_ICON: &[u8; 3251] = include_bytes!("../assets/desktop.png");

pub fn get_row<T: FromSql>(database: &path::Path, table: &str, row: &str) -> rusqlite::Result<T> {
    let conn = Connection::open(database)?;
    let mut stmt = conn.prepare(format!("SELECT {} FROM {};", row, table).as_str())?;
    let row: T = stmt.query_row([], |row| Ok(row.get(0)?))?;
    drop(stmt);
    conn.close().unwrap();
    Ok(row)
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

pub trait Serializable<T: Serialize + for<'de> Deserialize<'de>> {
    fn to_json(&self) -> io::Result<T>;
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

impl<T: Serialize + for<'de> Deserialize<'de>> Serializable<T> for &path::Path {
    fn to_json(&self) -> io::Result<T> {
        let data = std::fs::read_to_string(self)?;
        Ok(serde_json::from_str::<T>(&data)?)
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> Serializable<T> for path::PathBuf {
    fn to_json(&self) -> io::Result<T> {
        self.as_path().to_json()
    }
}

impl<T: Serialize + for<'de> Deserialize<'de>> Serializable<T> for String {
    fn to_json(&self) -> io::Result<T> {
        Ok(serde_json::from_str::<T>(&self)?)
    }
}
