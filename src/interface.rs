use std::{path, thread, time};

use rfd::FileDialog;
use slint::{ComponentHandle, Model, SharedString};

use crate::extract::{foreground_apps, get_icon, safe_canonicalize};
use crate::handler::BackgroundHandler;
use crate::load::Image;
use crate::types::Application;
use crate::{AppWindow, ProcessSlint};

pub struct Interface {
    pub ui: AppWindow,
    pub handler: BackgroundHandler,
}

impl Interface {
    /// Creates an Interface that helps manage the front-end of the application.
    ///
    /// This calls `BackgroundHandler::new`, so it should only be called during app initialization.
    pub fn new(ui: AppWindow) -> Self {
        let handler = BackgroundHandler::new(ui.as_weak().unwrap());
        Interface { ui, handler }
    }

    /// Creates a dummy Interface for the sake of easing other tasks.
    pub fn dummy(ui: AppWindow) -> Self {
        let handler = BackgroundHandler::dummy(ui.as_weak().unwrap());
        Interface { ui, handler }
    }

    /// Starts the application.
    pub fn start(&self) -> Result<&Self, slint::PlatformError> {
        self.handler.load_profiles();
        self.start_process_thread();
        self.ui.run()?;
        Ok(self)
    }

    /// Clears selected profile, name field, image field, and executable field.
    pub fn reset_fields(&self) -> () {
        self.ui.set_active_profile(slint::SharedString::new());
        self.ui.set_profile_field_name(slint::SharedString::new());
        self.ui.set_profile_field_img(slint::SharedString::new());
        self.ui.set_profile_field_exec(slint::SharedString::new());
    }

    /// Returns the currently selected application's index.
    pub fn displayed_profile(&self) -> Result<(Vec<Application>, usize), ()> {
        let data = self.handler.settings().unwrap().applications.applications;
        let profile_id = self.ui.get_active_profile();
        if profile_id == "" {
            return Err(());
        }
        let idx = &match data
            .iter()
            .position(|prof| prof.applicationId == profile_id.to_string())
        {
            Some(res) => res,
            None => return Err(()),
        };
        Ok((data, *idx))
    }

    /// Opens a File Explorer dialog to choose a file.
    pub fn select_file(
        &self,
        extension_name: &str,
        extensions: &[&str],
        directory: &path::Path,
    ) -> Result<String, ()> {
        let executable = FileDialog::new()
            .add_filter(extension_name, extensions)
            .set_directory(directory)
            .pick_file();

        match executable {
            Some(item) => {
                let path_str = item.to_string_lossy().to_string();
                Ok(path_str)
            }
            None => Err(()),
        }
    }

    fn start_process_thread(&self) {
        let reference = self.ui.as_weak();
        thread::spawn(move || {
            loop {
                let reference = reference.clone();
                slint::invoke_from_event_loop(move || {
                    let ui = reference.unwrap();
                    let needle = ui.get_search_text();

                    let running_processes =
                        unsafe { foreground_apps(&needle.to_lowercase().as_str()) };
                    let displayed_profiles = ui.get_profiles();
                    let slint_model = slint::VecModel::default();

                    for path in running_processes {
                        let filename = path.file_name().unwrap(); // We already verified this in foreground_apps
                        if displayed_profiles
                            .iter()
                            .any(|p| p.executable == &path.to_string_lossy())
                        {
                            continue;
                        }
                        let filepath = safe_canonicalize(path.as_path());
                        let img = Image::from_rgba(unsafe { get_icon(&filepath) });
                        let rgba = img.load_from_cache();
                        slint_model.push(ProcessSlint {
                            name: SharedString::from(filename.to_str().unwrap()),
                            executable: SharedString::from(filepath),
                            icon: slint::Image::from_rgba8(
                                slint::SharedPixelBuffer::clone_from_slice(
                                    rgba.as_raw(),
                                    rgba.width(),
                                    rgba.height(),
                                ),
                            ),
                        })
                    }
                    let displayed_processes = ui.get_processes();
                    let model_rc = slint::ModelRc::new(slint_model);
                    if displayed_processes.row_count() == model_rc.row_count() {
                        // No difference between displayed process list and actual process list.
                        // We don't have to update the UI.
                        return;
                    };
                    ui.set_processes(model_rc);
                })
                .unwrap();
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }
}

impl Clone for Interface {
    fn clone(&self) -> Self {
        Interface {
            ui: self.ui.as_weak().unwrap(),
            handler: self.handler.clone(),
        }
    }
}
