use std::path;

use slint::SharedString;

use crate::image::Image;
use crate::remapper::KeyboardKey;
use crate::types::Application;
use crate::utils::{APPLICATION_NAME_DESKTOP, DESKTOP_ICON};
use crate::{Game, GameType, Keybind, Process};

impl Keybind {
    pub fn new(index: i32, executable: String) -> Keybind {
        Keybind {
            index,
            executable: SharedString::from(executable),
            object: SharedString::from("NONE"),
            pointer: SharedString::from("NONE"),
            vkey_object: 0,
            vkey_pointer: 0,
        }
    }

    pub fn from_vkeys(index: i32, pointer: u64, object: u64, executable: String) -> Keybind {
        Keybind {
            index,
            pointer: SharedString::from(KeyboardKey::from(pointer).to_string()),
            object: SharedString::from(KeyboardKey::from(object).to_string()),
            executable: SharedString::from(executable),
            vkey_pointer: pointer as i32,
            vkey_object: object as i32,
        }
    }

    pub fn pointer_listening(other: Keybind) -> Keybind {
        Keybind {
            index: other.index,
            executable: other.executable,
            object: other.object,
            pointer: SharedString::from("..."),
            vkey_object: other.vkey_object,
            vkey_pointer: 0,
        }
    }

    pub fn object_listening(other: Keybind) -> Keybind {
        Keybind {
            index: other.index,
            executable: other.executable,
            object: SharedString::from("..."),
            pointer: other.pointer,
            vkey_object: 0,
            vkey_pointer: other.vkey_pointer,
        }
    }

    pub fn input_pointer(&self) -> KeyboardKey {
        KeyboardKey::from(self.vkey_pointer as u64)
    }

    pub fn input_object(&self) -> KeyboardKey {
        KeyboardKey::from(self.vkey_object as u64)
    }

    pub fn update_pointer(&self, pointer: u64) -> Keybind {
        Keybind::from_vkeys(
            self.index,
            pointer,
            self.vkey_object as u64,
            self.executable.to_string(),
        )
    }

    pub fn update_object(&self, object: u64) -> Keybind {
        Keybind::from_vkeys(
            self.index,
            self.vkey_pointer as u64,
            object,
            self.executable.to_string(),
        )
    }
}

impl Process {
    pub fn from_exec(exec: &path::Path) -> Process {
        Process {
            name: SharedString::from(exec.file_name().unwrap().to_string_lossy().to_string()),
            executable: SharedString::from(exec.to_string_lossy().to_string()),
            icon: Image::from(exec).to_slint(24, 24),
        }
    }
}

impl Game {
    pub fn from_settings(app: Application) -> Game {
        if app.isCustom.is_some() {
            let thumbnail_path = app.posterPath.unwrap();
            let thumbnail = Image::from(path::Path::new(&thumbnail_path));
            Game::custom(
                app.applicationId,
                app.name,
                thumbnail_path,
                app.applicationPath.unwrap(),
                thumbnail,
            )
        } else if app.name == APPLICATION_NAME_DESKTOP {
            Game::desktop()
        } else {
            let poster_url = app.posterUrl.unwrap();
            let banner = Image::from(poster_url.clone());
            Game::installed(
                app.applicationId,
                app.name,
                poster_url,
                app.applicationPath.unwrap_or(String::new()),
                banner,
            )
        }
    }

    pub fn desktop() -> Game {
        Game {
            id: SharedString::from(APPLICATION_NAME_DESKTOP),
            name: SharedString::from("Desktop"),
            image_path: SharedString::default(),
            executable: SharedString::default(),
            icon: Image::from(image::load_from_memory(DESKTOP_ICON).unwrap()).to_slint(48, 48),
            r#type: GameType::Desktop,
        }
    }

    pub fn installed(
        id: String,
        name: String,
        image_path: String,
        executable: String,
        icon: Image,
    ) -> Game {
        Game {
            id: SharedString::from(id),
            name: SharedString::from(name),
            image_path: SharedString::from(image_path),
            executable: SharedString::from(executable),
            icon: icon.to_slint(150, 210),
            r#type: GameType::Installed,
        }
    }

    pub fn custom(
        id: String,
        name: String,
        image_path: String,
        executable: String,
        icon: Image,
    ) -> Game {
        Game {
            id: SharedString::from(&id),
            name: SharedString::from(name),
            image_path: SharedString::from(image_path),
            executable: SharedString::from(executable),
            icon: icon.to_slint(48, 48),
            r#type: GameType::Custom,
        }
    }
}
