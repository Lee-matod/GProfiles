use std::path;

pub const GPROFILES: &str = "GPROFILES_APPLICATION_IDENTIFIER";
pub const APPLICATION_NAME_DESKTOP: &str = "APPLICATION_NAME_DESKTOP";
pub const PROFILE_NAME_DEFAULT: &str = "PROFILE_NAME_DEFAULT";
pub const BROKEN_IMAGE_ICON: &[u8; 5038] =
    include_bytes!("../assets/material_icons/broken_image.png");
pub const DESKTOP_ICON: &[u8; 3565] = include_bytes!("../assets/material_icons/desktop.png");
pub const APP_ICON: &[u8; 7032] = include_bytes!("../assets/app.ico");

pub fn logitech_folder() -> path::PathBuf {
    let localappdata = option_env!("LOCALAPPDATA").expect("no %localappdata% directory");

    let appdata_path = path::Path::new(localappdata);
    let ghub_folder = appdata_path.join("LGHUB");
    if !ghub_folder.exists() {
        panic!("LGHUB folder not in %localappdata%");
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
