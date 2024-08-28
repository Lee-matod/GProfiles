use slint::{ComponentHandle, Model};
use std::{path, process};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::extract::key_input;
use crate::image::Image;
use crate::remapper::KeyboardKey;
use crate::settings::{Commit, LogitechSettings};
use crate::types::Application;
use crate::utils::MessageBox;
use crate::{AppWindow, Game, GameType, Keybind};

pub fn restart_ghub() -> () {
    let refresh = ProcessRefreshKind::new().with_exe(sysinfo::UpdateKind::Always);
    let mut sys = System::new_with_specifics(RefreshKind::new().with_processes(refresh));
    sys.refresh_processes();
    for (_, proc) in sys.processes() {
        if let Some(exec) = proc.exe() {
            if exec.starts_with("C:\\Program Files\\LGHUB") {
                proc.kill();
            }
        }
    }
    match process::Command::new("C:\\Program Files\\LGHUB\\system_tray\\lghub_system_tray.exe")
        .spawn()
    {
        Ok(_) => {}
        Err(err) => {
            MessageBox::from(format!("Could not start LGHUB.\n{}", err.to_string())).warning();
        }
    };
}

pub fn new_application(app: AppWindow, executable: String) -> () {
    let settings = LogitechSettings::new();
    if let Some(application) = settings.create_application(path::Path::new(&executable)) {
        let game = Game::from_settings(application.clone());
        settings.commit(application);
        app.load_applications();
        app.set_active(game);
    }
    settings.close();
}

pub fn property_edit(
    app: AppWindow,
    ui_edit: impl FnOnce(&AppWindow, &mut Game) -> Option<Game> + 'static,
    settings_edit: impl FnOnce(&mut Application, Game) + Send + 'static,
) -> () {
    if let Some(active) = ui_edit(&app, &mut app.get_active_application()) {
        let settings = LogitechSettings::new();
        let mut application = match settings.app_from_game(active.clone()) {
            Some(app) => app,
            None => {
                MessageBox::from("The selected application could not be located. Maybe GProfiles and Logitech GHUB are out of sync?")
                .error();
                app.load_applications();
                settings.close();
                return;
            }
        };
        settings_edit(&mut application, active);
        settings.commit(application.clone());
        app.load_applications();
        app.set_active(Game::from_settings(application));
        settings.close();
    }
}

pub fn forget_application(app: AppWindow) -> () {
    let active = app.get_active_application();
    if active.r#type != GameType::Custom {
        return;
    }
    let image = Image::from(path::Path::new(&active.image_path.to_string()));
    let settings = LogitechSettings::new();
    let application = match settings.app_from_game(active) {
        Some(app) => app,
        None => {
            MessageBox::from(
                "The selected application could not be located.\nMaybe GProfiles and Logitech GHUB are out of sync?"
            ).error();
            app.load_applications();
            return;
        }
    };
    let profiles = settings.remove_profiles(&application);
    // This can be safely unwrapped as we already confirmed that the application existed.
    let applications = settings.remove_application(application).unwrap();
    image.delete();
    settings.commit(applications);
    settings.commit(profiles);
    settings.close();
    app.load_applications();
    app.set_active(Game::desktop());
}

pub fn new_key(app: AppWindow) -> () {
    let active = app.get_active_application();

    let keybinds = app.get_keybinds();
    let rc: slint::VecModel<Keybind> = slint::VecModel::default();

    keybinds.iter().for_each(|item| rc.push(item));
    rc.push(Keybind::new(
        keybinds.row_count() as i32 + 1,
        active.executable.to_string(),
    ));
    app.set_keybinds(slint::ModelRc::new(rc));
}

pub fn set_key(
    app: AppWindow,
    keybind: Keybind,
    get_keymap: impl FnOnce(&Keybind) -> Keybind + 'static,
    update_keymap: impl FnOnce(Keybind, KeyboardKey) -> Keybind + Send + 'static,
    get_key: impl FnOnce(&Keybind) -> KeyboardKey + Send + 'static,
) -> () {
    app.set_keymap(get_keymap(&keybind));

    unsafe {
        key_input({
            let weak = app.as_weak();
            move |key| {
                let app = weak.unwrap();

                if key.is_none() {
                    app.set_keymap(keybind);
                    return;
                }
                let key = key.unwrap();
                let keybind = update_keymap(keybind, key);
                if get_key(&keybind) != KeyboardKey::Escape {
                    let mut keybinds = Vec::from_iter(app.get_keybinds().iter());
                    keybinds.push(keybind.clone());
                    let executable = app.get_active_application_executable().to_string();

                    let settings = LogitechSettings::new();
                    settings.set_keybinds(executable, keybinds);
                    settings.close();
                }
                app.set_keymap(keybind);
            }
        })
    }
}

pub fn delete_key(app: AppWindow, keybind: Keybind) -> () {
    let settings = LogitechSettings::new();
    settings.remove_keybind(&keybind);
    settings.close();
    app.load_keymaps(&app.get_active_application());
}
