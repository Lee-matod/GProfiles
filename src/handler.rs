use std::{path, process};

use serde_json::Value;
use slint::{ComponentHandle, SharedPixelBuffer, SharedString};
use uuid::Uuid;

use crate::callbacks::{
    on_add_app, on_exec_edit, on_forget_app, on_image_edit, on_name_edit, wrapper,
};
use crate::extract::get_icon;
use crate::interface::Interface;
use crate::load::Image;
use crate::processes::{resolve_path, system};
use crate::types::{Application, InnerApplications, InnerProfiles, JsonData, Profile};
use crate::{AppWindow, ProfileSlint};

const BROKEN_IMAGE: &[u8; 565] = include_bytes!("../assets/material_icons/broken_image_48.png");

pub struct BackgroundHandler {
    pub ui: AppWindow,
}

impl BackgroundHandler {
    pub fn new(ui: AppWindow) -> Self {
        let handler = BackgroundHandler::dummy(ui);
        handler.set_callbacks();
        handler
    }

    pub fn dummy(ui: AppWindow) -> Self {
        BackgroundHandler { ui }
    }

    pub fn settings(&self) -> rusqlite::Result<JsonData> {
        let ghub_settings = self.get_database();
        let conn = rusqlite::Connection::open(ghub_settings)?;
        let mut stmt = conn.prepare("SELECT file FROM data")?;
        let row: Vec<u8> = stmt.query_row([], |row| Ok(row.get(0)?))?;
        let decoded = String::from_utf8_lossy(&row).to_string();
        Ok(serde_json::from_str(&decoded).unwrap())
    }

    pub fn load_profiles(&self) -> () {
        let mut profiles: Vec<ProfileSlint> = Vec::new();
        let data = self.settings().unwrap().applications.applications;
        for app in data.iter() {
            if app.isCustom.is_none() {
                continue;
            }

            let image_dir = app.posterPath.clone().unwrap();
            let image_path = path::Path::new(&image_dir);
            let icon: slint::Image;
            if image_path.exists() && image_path.extension().is_some_and(|e| e == "bmp") {
                let rgba = Image::from_path(&image_path).load_from_cache();
                icon = slint::Image::from_rgba8(slint::SharedPixelBuffer::clone_from_slice(
                    rgba.as_raw(),
                    rgba.width(),
                    rgba.height(),
                ))
            } else {
                icon = slint::Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
                    BROKEN_IMAGE,
                    48,
                    48,
                ))
            }
            profiles.push(ProfileSlint {
                id: SharedString::from(&app.applicationId),
                name: SharedString::from(&app.name),
                executable: SharedString::from(&app.applicationPath.clone().unwrap()),
                image_path: SharedString::from(&image_dir),
                icon,
            });
        }
        self.ui
            .set_profiles(slint::ModelRc::new(slint::VecModel::from(profiles)));
    }

    pub fn commit(&self, applications: Vec<Application>, profiles: Option<Vec<Profile>>) -> () {
        let ghub_settings = self.get_database();
        let conn = rusqlite::Connection::open(ghub_settings).unwrap();
        let mut stmt = conn.prepare("SELECT file FROM data").unwrap();
        let row: Vec<u8> = stmt.query_row([], |row| Ok(row.get(0)?)).unwrap();
        let decoded = String::from_utf8_lossy(&row).to_string();
        let mut raw: Value = serde_json::from_str(&decoded).unwrap();

        raw["applications"]["applications"] =
            InnerApplications { applications }.try_into().unwrap();
        if profiles.is_some() {
            raw["profiles"]["profiles"] = InnerProfiles {
                profiles: profiles.unwrap(),
            }
            .try_into()
            .unwrap();
        }

        let ghub = self.get_database();
        let conn = rusqlite::Connection::open(ghub).unwrap();
        let _ = conn.execute(
            "UPDATE data SET file=?",
            (serde_json::to_string(&raw).unwrap().as_bytes(),),
        );
        let _ = conn.close().unwrap();
        self.load_profiles();
    }

    pub fn get_desktop_profile(&self) -> Option<Application> {
        let settings = self.settings().unwrap();
        for app in settings.applications.applications {
            if app.name == "APPLICATION_NAME_DESKTOP" {
                return Some(app);
            }
        }
        None
    }

    pub fn find_profiles(&self, app: &Application) -> Vec<Profile> {
        let settings = self.settings().unwrap();
        let mut matched_profiles: Vec<Profile> = Vec::new();
        for profile in settings.profiles.profiles {
            if profile.applicationId == app.applicationId {
                matched_profiles.push(profile);
            }
        }
        matched_profiles
    }

    pub fn create_application(&self, exec: &path::Path) -> (Vec<Application>, Vec<Profile>) {
        let mut settings = self.settings().unwrap();
        let app_uuid = Uuid::new_v4().to_string().replace("-", "");

        let buffer = unsafe { get_icon(exec.to_str().unwrap()) };
        let img = Image::from_rgba(buffer);
        let icon = img.save_to_cache(app_uuid.clone());

        let desktop = self.get_desktop_profile().expect("no desktop profile");
        let desktop_profiles = self.find_profiles(&desktop);
        let default_profile = desktop_profiles
            .iter()
            .find(|p| p.name == "PROFILE_NAME_DEFAULT")
            .expect("no default profile");

        let app = Application {
            name: exec.file_stem().unwrap().to_string_lossy().to_string(),
            applicationId: app_uuid,
            applicationPath: Some(resolve_path(exec)),
            isCustom: Some(true),
            posterPath: Some(resolve_path(icon.as_path())),
        };
        let profile = Profile {
            activeForApplication: true,
            applicationId: app.applicationId.clone(),
            id: app.applicationId.clone(),
            name: String::from("PROFILE_NAME_DEFAULT"),
            assignments: default_profile.assignments.clone(),
        };
        settings.applications.applications.push(app);
        settings.profiles.profiles.push(profile);
        (
            settings.applications.applications,
            settings.profiles.profiles,
        )
    }

    fn get_database(&self) -> path::PathBuf {
        let localappdata_env = option_env!("LOCALAPPDATA").expect("no %localappdata% directory");

        let localappdata = path::Path::new(localappdata_env);
        let ghub_folder = localappdata.join("LGHUB");
        if !ghub_folder.exists() {
            panic!("LGHUB folder not in %localappdata%");
        }
        let ghub_settings = ghub_folder.join("settings.db");
        if !ghub_settings.exists() {
            panic!("settings.db not in LGHUB");
        }
        ghub_settings
    }

    fn set_callbacks(&self) {
        // I strongly disapprove of this
        let weak = self.ui.as_weak();
        self.ui.on_name_edit(move || {
            let ui = weak.unwrap();
            let interface = Interface::dummy(ui);
            wrapper(&interface, on_name_edit)
        });
        let weak = self.ui.as_weak();
        self.ui.on_image_edit(move || {
            let ui = weak.unwrap();
            let interface = Interface::dummy(ui);
            wrapper(&interface, on_image_edit)
        });
        let weak = self.ui.as_weak();
        self.ui.on_exec_edit(move || {
            let ui = weak.unwrap();
            let interface = Interface::dummy(ui);
            wrapper(&interface, on_exec_edit)
        });
        let weak = self.ui.as_weak();
        self.ui.on_add_app(move || {
            let ui = weak.unwrap();
            let interface = Interface::dummy(ui);
            wrapper(&interface, on_add_app)
        });
        let weak = self.ui.as_weak();
        self.ui.on_forget_app(move || {
            let ui = weak.unwrap();
            let interface = Interface::dummy(ui);
            wrapper(&interface, on_forget_app)
        });
        let weak = self.ui.as_weak();
        self.ui.on_add_process(move |process| {
            let ui = weak.unwrap();
            let handler = BackgroundHandler::dummy(ui);
            let (apps, profiles) =
                handler.create_application(path::Path::new(&process.executable.to_string()));
            handler.commit(apps, Some(profiles))
        });
        self.ui.on_restart_ghub(move || {
            let mut sys = system();
            sys.refresh_processes();
            for (_, proc) in sys.processes() {
                let exec = match proc.exe() {
                    Some(res) => res,
                    None => continue,
                };
                if exec.starts_with("C:\\Program Files\\LGHUB") {
                    proc.kill();
                }
            }
            process::Command::new("C:\\Program Files\\LGHUB\\system_tray\\lghub_system_tray.exe")
                .spawn()
                .unwrap();
        })
    }
}

impl Clone for BackgroundHandler {
    fn clone(&self) -> Self {
        BackgroundHandler {
            ui: self.ui.as_weak().unwrap(),
        }
    }
}
