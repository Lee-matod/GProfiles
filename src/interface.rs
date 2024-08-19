use std::path::{self, PathBuf};
use std::thread;

use image;
use rfd::FileDialog;
use slint::{ComponentHandle, Model, Weak};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, UnhookWindowsHookEx};

use crate::extract::{foreground_apps, hwnd_path};
use crate::remapper;
use crate::remapper::listener;
use crate::settings::LogitechSettings;
use crate::types::Application;
use crate::utils::{MessageBox, APP_ICON};
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
        let settings = LogitechSettings::new();
        let apps: Vec<Application> = settings.get_applications();
        settings.close();

        let active = self.get_active_application();
        if let Some(app) = apps
            .iter()
            .find(|item| item.applicationId == active.id.to_string())
        {
            return Some(app.clone());
        }
        None
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
            return Some(path_str);
        }
        None
    }

    pub fn load_applications(&self) -> () {
        let settings = LogitechSettings::new();
        let applications = settings.get_applications();
        settings.close();
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

    pub fn load_keymaps(&self, application: &Game) -> () {
        let settings = LogitechSettings::new();
        let keybinds = settings.get_keybinds(&application.executable.to_string());
        settings.close();
        let rc: slint::VecModel<Keybind> = slint::VecModel::default();
        for key in keybinds {
            if let Ok(key) = key {
                rc.push(key)
            }
        }
        self.set_keybinds(slint::ModelRc::new(rc));
    }

    pub fn start(&self, mutex_handle: HANDLE) -> () {
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
            move || AppWindow::background_loop(weak)
        });

        self.load_applications();
        self.load_keymaps(&Game::desktop());

        self.show().unwrap();
        slint::run_event_loop_until_quit().unwrap();
        self.hide().unwrap();
        unsafe { CloseHandle(mutex_handle).unwrap() };
    }

    fn background_loop(weak: Weak<AppWindow>) -> () {
        // 4 things happen in this one tiny function:
        //
        // 1. Keybind hook is set and kept active until the application is quit.
        // 2. Active foreground window is actively queried, which then updates which keymap layout to use.
        // 3. Any system tray events are handled.
        // 4. Running process list is updated.

        let h_hook = match unsafe { listener::set_hook() } {
            Ok(hook) => Some(hook),
            Err(err) => {
                // We don't return here because this simple thing is not worthy enough to prevent the entire
                // application from working.
                MessageBox::from_error("Could not set keybind hook.", err.to_string()).warning();
                None
            }
        };

        // While the application is still alive, we should continue this loop
        while AppWindow::handle_trayicon(weak.clone()) {
            unsafe {
                let hwnd = GetForegroundWindow();
                let active_window = hwnd_path(hwnd);
                if active_window.is_some() {
                    remapper::set_keymap(&active_window.unwrap().to_string_lossy().to_string());
                }
            }
            let foreground_apps = unsafe { foreground_apps() };

            let weak = weak.clone();
            slint::invoke_from_event_loop(move || {
                // We don't have to waste time in this if the application is not shown anyway.
                let app = weak.unwrap();
                if !app.window().is_visible() {
                    return;
                }

                // Get foreground applications matching the query.
                let query = app.get_process_query().to_string();
                let foreground: Vec<&PathBuf> = foreground_apps
                    .iter()
                    .filter(|i| i.file_name().unwrap().to_string_lossy().starts_with(&query))
                    .collect();

                // Filter processes that have already been added.
                let apps = app.get_applications();
                let processes: slint::VecModel<Process> = slint::VecModel::default();
                for proc in foreground.iter() {
                    if apps.iter().any(|p| {
                        !p.executable.is_empty()
                            && proc
                                .to_string_lossy()
                                .starts_with(&p.executable.to_string())
                    }) {
                        continue;
                    }
                    processes.push(Process::from_exec(proc.as_path()));
                }

                // Finally, check if there are any changes and push.
                let current = app.get_processes();
                if current.row_count() == processes.row_count() {
                    return;
                }
                app.set_processes(slint::ModelRc::new(processes));
            })
            .unwrap()
        }

        if h_hook.is_some() {
            unsafe { UnhookWindowsHookEx(h_hook.unwrap()).unwrap() };
        }
    }

    fn handle_trayicon(weak: Weak<AppWindow>) -> bool {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id.0 == "1000" {
                let weak = weak.clone();
                slint::invoke_from_event_loop(move || weak.unwrap().show().unwrap()).unwrap()
            } else {
                slint::quit_event_loop().unwrap();
                return false;
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
                        slint::invoke_from_event_loop(move || weak.unwrap().show().unwrap())
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
        return true;
    }
}
