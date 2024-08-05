use slint::{ComponentHandle, Model};
use std::{path, process};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::extract::key_input;
use crate::remapper::KeyboardKey;
use crate::settings::{Commit, LogitechSettings};
use crate::types::Application;
use crate::{AppWindow, Game, GameType, Keybind, Process};

pub struct Callbacks;

impl Callbacks {
    pub fn application_clicked(app: AppWindow, application: Game) -> () {
        app.load_keymaps(&application);
        app.set_application(application);
    }

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
        process::Command::new("C:\\Program Files\\LGHUB\\system_tray\\lghub_system_tray.exe")
            .spawn()
            .unwrap();
    }

    pub fn from_executable(app: AppWindow) -> () {
        let executable = app.select_file("Executable", &["exe"], path::Path::new(""));
        if executable.is_none() {
            return;
        }
        let settings = LogitechSettings::new();
        if let Some(application) =
            settings.create_application(path::Path::new(&executable.unwrap()))
        {
            let game = Game::from_settings(application.clone());
            settings.commit(application);
            app.load_applications();
            app.load_keymaps(&game);
            app.set_application(game);
        }
        settings.close();
    }

    pub fn from_process(app: AppWindow, process: Process) -> () {
        let settings = LogitechSettings::new();
        if let Some(application) =
            settings.create_application(path::Path::new(&process.executable.to_string()))
        {
            let game = Game::from_settings(application.clone());
            app.load_applications();
            app.load_keymaps(&game);
            app.set_application(game);
        }
        settings.close();
    }

    pub fn name_edit(app: AppWindow) -> () {
        let settings = LogitechSettings::new();
        let active = app.get_active_application();
        let name = app.get_active_application_name();
        let mut application = settings.app_from_game(active);
        application.name = name.to_string();
        let game = Game::from_settings(application.clone());
        settings.commit(application);
        settings.close();
        app.load_applications();
        app.set_application(game);
    }

    pub fn file_edit(
        app: AppWindow,
        implementation: impl FnOnce(&AppWindow) -> Option<String> + 'static,
        handler: impl FnOnce(&mut Application, String),
    ) -> () {
        let image_path = match implementation(&app) {
            Some(p) => p,
            None => return,
        };

        let active = app.get_active_application();
        let settings = LogitechSettings::new();
        let mut application = settings.app_from_game(active);
        handler(&mut application, image_path);
        let game = Game::from_settings(application.clone());
        let applications = settings.update_application(&application);
        settings.commit(applications);
        app.load_applications();
        settings.close();
        app.set_application(game);
    }

    pub fn forget_application(app: AppWindow) -> () {
        let active = app.get_active_application();
        if active.r#type != GameType::Custom {
            return;
        }
        let settings = LogitechSettings::new();
        let application = settings.app_from_game(active);
        let applications = settings.remove_application(application);
        settings.commit(applications);
        settings.close();

        let desktop = Game::desktop();
        app.load_applications();
        app.load_keymaps(&desktop);
        app.set_application(desktop);
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

    pub fn set_pointer(app: AppWindow, keybind: Keybind) -> () {
        app.set_keymap(Keybind::pointer_listening(keybind.clone()));

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
                    let keybind = keybind.update_pointer(u64::from(&key));
                    if keybind.input_object() != KeyboardKey::Escape {
                        let settings = LogitechSettings::new();
                        settings.add_keybind(&keybind);
                        settings.close();
                    }
                    app.set_keymap(keybind);
                }
            })
        }
    }

    pub fn set_object(app: AppWindow, keybind: Keybind) -> () {
        app.set_keymap(Keybind::object_listening(keybind.clone()));

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
                    let keybind = keybind.update_object(u64::from(&key));
                    if keybind.input_pointer() != KeyboardKey::Escape {
                        let settings = LogitechSettings::new();
                        settings.add_keybind(&keybind);
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
}
