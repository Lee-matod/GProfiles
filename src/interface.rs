use std::{path, thread, time};

use rfd::FileDialog;
use slint::{ComponentHandle, Model, SharedString};

use crate::extract::get_icon;
use crate::handler::BackgroundHandler;
use crate::load::Image;
use crate::processes::{get_processes, resolve_path};
use crate::types::Application;
use crate::{AppWindow, ProcessSlint};

pub struct Interface {
    pub ui: AppWindow,
    pub handler: BackgroundHandler,
}

impl Interface {
    pub fn new(ui: AppWindow) -> Self {
        let handler = BackgroundHandler::new(ui.as_weak().unwrap());
        Interface { ui, handler }
    }

    pub fn dummy(ui: AppWindow) -> Self {
        let handler = BackgroundHandler::dummy(ui.as_weak().unwrap());
        Interface { ui, handler }
    }

    pub fn start(&self) -> Result<&Self, slint::PlatformError> {
        self.handler.load_profiles();
        self.start_process_thread();
        self.ui.run()?;
        Ok(self)
    }

    pub fn reset_fields(&self) -> () {
        self.ui.set_active_profile(slint::SharedString::new());
        self.ui.set_profile_field_name(slint::SharedString::new());
        self.ui.set_profile_field_img(slint::SharedString::new());
        self.ui.set_profile_field_exec(slint::SharedString::new());
    }

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

                    let running_processes = get_processes(&needle.to_lowercase().as_str());
                    let displayed_processes = ui.get_processes();
                    let displayed_profiles = ui.get_profiles();

                    let mut to_slint: Vec<ProcessSlint> = Vec::new();
                    for path in running_processes {
                        let filename = path.file_name();
                        if displayed_profiles
                            .iter()
                            .any(|p| p.executable.to_string() == path.to_string_lossy().to_string())
                            || filename.is_none()
                        {
                            continue;
                        }
                        let filepath = resolve_path(path.as_path());
                        let img = Image::from_rgba(unsafe { get_icon(&filepath) });
                        let rgba = img.load_from_cache();
                        to_slint.push(ProcessSlint {
                            name: SharedString::from(
                                filename.unwrap().to_string_lossy().to_string(),
                            ),
                            executable: SharedString::from(filepath),
                            icon: slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(
                                rgba.as_raw(),
                                rgba.width(),
                                rgba.height(),
                            )),
                        })
                    }
                    if to_slint.len() == displayed_processes.iter().len() {
                        // No difference between displayed process list and actual process list.
                        // We don't have to update the UI.
                        return;
                    };
                    ui.set_processes(slint::ModelRc::new(slint::VecModel::from(to_slint)));
                })
                .unwrap();
                thread::sleep(time::Duration::from_millis(100));
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
