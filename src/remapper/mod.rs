mod keyboard;
pub mod listener;

use std::collections::HashMap;
use std::sync::RwLock;

use crate::settings::LogitechSettings;
pub use keyboard::KeyboardKey;

static mut ACTIVE_KEYMAP: RwLock<Option<HashMap<u16, u16>>> = RwLock::new(None);

pub fn set_keymap(executable: &String) -> () {
    let settings = LogitechSettings::new();
    let application = match settings.get_application(executable) {
        Some(app) => app,
        None => settings.get_desktop_application(),
    };
    let keybinds = settings.get_keybinds(&application.applicationPath.unwrap_or(String::new()));
    settings.close();

    let mut new_keymap: HashMap<u16, u16> = HashMap::new();
    for keybind in keybinds {
        if let Ok(key) = keybind {
            new_keymap.insert(key.vkey_pointer as u16, key.vkey_object as u16);
        }
    }

    unsafe { ACTIVE_KEYMAP = RwLock::new(Some(new_keymap)) };
}
