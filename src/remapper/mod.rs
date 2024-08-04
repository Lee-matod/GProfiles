mod keyboard;
pub mod listener;

use std::{collections::HashMap, sync::RwLock};

use crate::settings::LogitechSettings;
pub use keyboard::KeyboardKey;

static mut ACTIVE_KEYMAP: RwLock<Option<HashMap<u16, u16>>> = RwLock::new(None);

pub fn set_keymap(executable: &String) -> () {
    let settings = LogitechSettings::new();
    let keybinds = settings.get_keybinds(executable);

    let mut new_keymap: HashMap<u16, u16> = HashMap::new();
    for keybind in keybinds {
        if let Ok(key) = keybind {
            new_keymap.insert(key.vkey_pointer as u16, key.vkey_object as u16);
        }
    }
    settings.close();

    unsafe { ACTIVE_KEYMAP = RwLock::new(Some(new_keymap)) }
}
