use std::path;

use windows::core::PCWSTR;

use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, MESSAGEBOX_STYLE,
};

use crate::extract::encode_wide;

pub const GPROFILES: &str = "GPROFILES_APPLICATION_IDENTIFIER";
pub const APPLICATION_NAME_DESKTOP: &str = "APPLICATION_NAME_DESKTOP";
pub const PROFILE_NAME_DEFAULT: &str = "PROFILE_NAME_DEFAULT";
pub const BROKEN_IMAGE_ICON: &[u8; 5038] =
    include_bytes!("../assets/material_icons/broken_image.png");
pub const DESKTOP_ICON: &[u8; 3565] = include_bytes!("../assets/material_icons/desktop.png");
pub const APP_ICON: &[u8; 7032] = include_bytes!("../assets/app.ico");

pub fn logitech_folder() -> path::PathBuf {
    let localappdata = match option_env!("LOCALAPPDATA") {
        Some(path) => path,
        None => {
            MessageBox::from(
                "No LOCALAPPDATA environment variable found. Most likely it simply does not exist.",
            )
            .error();
            panic!()
        }
    };

    let appdata_path = path::Path::new(localappdata);
    let ghub_folder = appdata_path.join("LGHUB");
    if !ghub_folder.exists() {
        MessageBox::from("%LOCALAPPDATA%\\LGHUB is not a directory.").error();
        panic!();
    }
    ghub_folder
}

pub fn safe_canonicalize(path: &path::Path) -> String {
    let canon = match path.canonicalize() {
        Ok(pathbuf) => pathbuf,
        Err(_) => return path.to_string_lossy().to_string(),
    };
    let full_path = canon.to_string_lossy().to_string();
    match full_path.strip_prefix("\\\\?\\") {
        Some(stripped) => stripped.to_string(),
        None => full_path,
    }
}

pub enum MessageBoxResult {
    Abort,
    Cancel,
    Continue,
    Ignore,
    No,
    OK,
    Retry,
    TryAgain,
    Yes,
    Invalid,
}

impl From<i32> for MessageBoxResult {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::OK,
            2 => Self::Cancel,
            3 => Self::Abort,
            4 => Self::Retry,
            5 => Self::Ignore,
            6 => Self::Yes,
            7 => Self::No,
            10 => Self::TryAgain,
            11 => Self::Continue,
            _ => Self::Invalid,
        }
    }
}

pub struct MessageBox {
    text: String,
}

impl MessageBox {
    pub fn info(&self) -> MessageBoxResult {
        MessageBoxResult::from(self.display(MB_OK | MB_ICONINFORMATION))
    }

    pub fn warning(&self) -> MessageBoxResult {
        MessageBoxResult::from(self.display(MB_OK | MB_ICONWARNING))
    }

    pub fn error(&self) -> MessageBoxResult {
        MessageBoxResult::from(self.display(MB_OK | MB_ICONERROR))
    }

    fn display(&self, utype: MESSAGEBOX_STYLE) -> i32 {
        unsafe {
            MessageBoxW(
                None,
                PCWSTR::from_raw(encode_wide(&self.text).as_ptr()),
                PCWSTR::from_raw(encode_wide("GProfiles").as_ptr()),
                utype,
            )
            .0
        }
    }
}

impl From<String> for MessageBox {
    fn from(value: String) -> Self {
        MessageBox { text: value }
    }
}

impl From<&'static str> for MessageBox {
    fn from(value: &'static str) -> Self {
        MessageBox {
            text: value.to_string(),
        }
    }
}
