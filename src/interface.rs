use std::path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use image;
use rfd::FileDialog;
use slint::{ComponentHandle, Model, Weak};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::{CloseHandle, HANDLE};

use crate::extract::foreground_apps;
use crate::remapper;
use crate::remapper::listener;
use crate::settings::LogitechSettings;
use crate::types::Application;
use crate::utils::APP_ICON;
use crate::{AppWindow, Game, GameType, Keybind, Process};

impl AppWindow {
    pub fn set_application(&self, game: Game) -> () {
        self.set_active_application(game.clone());
        self.set_active_application_name(game.name);
        self.set_active_application_image(game.image_path);
        self.set_active_application_executable(game.executable);
        self.set_active_application_type(game.r#type);
    }

    pub fn selected_application(&self) -> Option<Application> {
        let settings = LogitechSettings::new()?;
        let apps: Vec<Application> = settings.get_applications();
        settings.close()?;

        let active = self.get_active_application();
        if let Some(app) = apps
            .iter()
            .find(|item| item.applicationId == active.id.to_string())
        {
            Some(app.clone())
        } else {
            None
        }
    }

    pub fn select_file(
        &self,
        extension_name: &str,
        extensions: &[&str],
        directory: &path::Path,
    ) -> Option<String> {
        let executable = FileDialog::new()
            .add_filter(extension_name, extensions)
            .set_directory(directory)
            .pick_file();

        if let Some(item) = executable {
            let path_str = item.to_string_lossy().to_string();
            Some(path_str)
        } else {
            None
        }
    }

    pub fn load_applications(&self) -> Option<()> {
        let settings = LogitechSettings::new()?;
        let applications = settings.get_applications();
        let games: slint::VecModel<Game> = slint::VecModel::default();
        for app in applications.iter() {
            let component = Game::from_settings(app.clone());
            if component.r#type == GameType::Desktop {
                games.insert(0, component);
            } else {
                games.push(component);
            }
        }
        self.set_applications(slint::ModelRc::new(games));
        settings.close()?;
        Some(())
    }

    pub fn load_processes(&self, query: &str) -> () {
        let foreground = unsafe { foreground_apps(query) };
        let apps = self.get_applications();
        let processes: slint::VecModel<Process> = slint::VecModel::default();
        for proc in foreground {
            if apps.iter().any(|p| p.executable == proc.to_string_lossy()) {
                continue;
            }
            processes.push(Process::from_exec(proc.as_path()));
        }
        self.set_processes(slint::ModelRc::new(processes));
    }

    pub fn set_keymap(&self, keybind: Keybind) -> () {
        let current_model = self.get_keybinds();
        let mut current = Vec::from_iter(current_model.iter());
        match current.iter().position(|item| item.index == keybind.index) {
            Some(index) => {
                current.remove(index);
                current.insert(index, keybind);
            }
            None => {
                current.push(keybind);
            }
        };
        self.set_keybinds(slint::ModelRc::new(slint::VecModel::from(current)));
    }

    pub fn load_keymaps(&self, application: &Game) -> Option<()> {
        let settings = LogitechSettings::new()?;
        let keybinds = settings
            .get_keybinds(&application.executable.to_string())
            .ok()?;
        let rc: slint::VecModel<Keybind> = slint::VecModel::default();
        for key in keybinds {
            if let Ok(key) = key {
                rc.push(key)
            }
        }
        self.set_keybinds(slint::ModelRc::new(rc));
        settings.close()?;
        Some(())
    }

    pub fn start(&self, mutex_handle: HANDLE) -> () {
        // Process updating thread
        thread::spawn({
            let weak = self.as_weak();
            move || AppWindow::background_task(weak)
        });

        // Keymapping thread
        thread::spawn(move || unsafe {
            listener::set_hook().unwrap();
        });

        // Tray icon thread
        let rgba = image::load_from_memory(APP_ICON).unwrap().to_rgba8();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(
                Menu::with_items(&[
                    &MenuItem::new("Open", true, None),
                    &MenuItem::new("Quit", true, None),
                ])
                .unwrap(),
            ))
            .with_tooltip("GProfiles")
            .with_icon(Icon::from_rgba(rgba.as_raw().clone(), rgba.width(), rgba.height()).unwrap())
            .build()
            .unwrap();

        thread::spawn({
            let weak = self.as_weak();
            move || loop {
                if let Ok(event) = MenuEvent::receiver().try_recv() {
                    if event.id.0 == "1000" {
                        let weak = weak.clone();
                        slint::invoke_from_event_loop(move || weak.unwrap().show().unwrap())
                            .unwrap()
                    } else {
                        slint::quit_event_loop().unwrap()
                    }
                }
                if let Ok(event) = TrayIconEvent::receiver().try_recv() {
                    match event {
                        TrayIconEvent::Click {
                            id: _,
                            position: _,
                            rect: _,
                            button,
                            button_state,
                        } => {
                            if button == MouseButton::Left && button_state == MouseButtonState::Up {
                                let weak = weak.clone();
                                slint::invoke_from_event_loop(move || {
                                    weak.unwrap().show().unwrap()
                                })
                                .unwrap();
                            }
                        }
                        _ => continue,
                    }
                }
            }
        });

        self.load_applications().unwrap();
        self.load_processes("");
        self.load_keymaps(&Game::desktop()).unwrap();

        self.show().unwrap();
        slint::run_event_loop_until_quit().unwrap();
        self.hide().unwrap();
        unsafe { CloseHandle(mutex_handle).unwrap() };
    }

    fn background_task(weak: Weak<AppWindow>) -> () {
        loop {
            let (sender, receiver) = mpsc::channel();
            slint::invoke_from_event_loop({
                let weak = weak.clone();
                move || {
                    let app = weak.unwrap();
                    sender.send(app.get_process_query().to_string()).unwrap();
                }
            })
            .unwrap();
            let query = receiver.recv().unwrap();

            let foreground = unsafe { foreground_apps(&query) };

            slint::invoke_from_event_loop({
                let weak = weak.clone();
                let foreground = foreground.clone();
                move || {
                    let app = weak.unwrap();
                    let apps = app.get_applications();

                    let processes: slint::VecModel<Process> = slint::VecModel::default();
                    for proc in foreground.iter() {
                        if apps.iter().any(|p| {
                            !p.executable.is_empty()
                                && proc
                                    .to_string_lossy()
                                    .to_string()
                                    .starts_with(&p.executable.to_string())
                        }) {
                            continue;
                        }
                        processes.push(Process::from_exec(proc.as_path()));
                    }

                    app.set_processes(slint::ModelRc::new(processes));
                }
            })
            .unwrap();

            if let Some(top) = foreground.get(0) {
                remapper::set_keymap(&top.to_string_lossy().to_string()).unwrap();
            }

            thread::sleep(Duration::from_millis(500))
        }
    }
}
