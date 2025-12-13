use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs, io, path,
    sync::{OnceLock, RwLock},
};

use crate::{
    types::{
        gprofiles::{GProfilesData, Keybind},
        logitech::{Application, LogitechData, Profile},
    },
    utils::{APPLICATION_NAME_DESKTOP, parse_json},
};

pub static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

pub fn get_config() -> &'static RwLock<Config> {
    CONFIG.get_or_init(|| RwLock::new(Config::new()))
}

fn get_default_storage(identifier: &str, data: Option<&str>) -> io::Result<path::PathBuf> {
    let parent_dir: Option<OsString> = if cfg!(target_os = "windows") {
        env::var_os("LOCALAPPDATA")
    } else if cfg!(target_os = "linux") {
        env::var_os("XDG_CONFIG_HOME")
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "OS not supported.",
        ));
    };
    if parent_dir.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Configuration directory not set correctly.",
        ));
    }
    let parent = path::PathBuf::from(&parent_dir.unwrap());
    if !parent.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Configuration directory does not exist.",
        ));
    }

    let target = parent.join(identifier);
    if let Some(obj) = data {
        if !target.exists() {
            fs::create_dir(&target)?;
        }
        let items: Vec<&str> = obj.split(" ").collect();
        let (name, data) = (items[0], &items[1..]);
        let child = target.join(name);
        if !child.exists() {
            fs::write(&child, data.join(" "))?;
        }
        return Ok(child);
    }
    Ok(target)
}

#[derive(Debug)]
pub struct Config {
    applications: Vec<Application>,
    profiles: Vec<Profile>,
    keybinds: HashMap<String, Vec<Keybind>>,
    gprofiles_settings: path::PathBuf,
    lghub_location: path::PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let gprofiles_settings =
            get_default_storage("GProfiles", Some("settings.json {}")).unwrap();
        let gprofiles_data: GProfilesData = parse_json(&gprofiles_settings);

        let lghub_location = if gprofiles_data.settings.is_none() {
            get_default_storage("LGHUB", None).unwrap()
        } else {
            path::PathBuf::from(&gprofiles_data.settings.unwrap()) // safe
        };
        let logitech_data: LogitechData = parse_json(&lghub_location);

        let applications = logitech_data.applications.applications;
        let profiles = logitech_data.profiles.profiles;
        let keybinds = gprofiles_data.keybinds.unwrap_or_default();

        Self {
            applications,
            profiles,
            gprofiles_settings,
            lghub_location,
            keybinds,
        }
    }

    pub fn get_applications(&self) -> &Vec<Application> {
        &self.applications
    }

    pub fn edit_application<P: Fn(Application) -> Application>(&mut self, id: &String, pred: P) {
        if let Some(index) = self.get_application_index(id) {
            let new = pred(self.applications[index].clone());
            self.applications[index] = new;
        }
    }

    pub fn replace_application(&mut self, id: &String, app: Application) {
        if let Some(index) = self.get_application_index(id) {
            self.applications[index] = app;
        }
    }

    pub fn get_application(&self, id: &String) -> Option<&Application> {
        self.applications.iter().find(|a| &a.applicationId == id)
    }

    pub fn get_desktop_application(&self) -> Option<&Application> {
        self.applications
            .iter()
            .find(|a| a.name == APPLICATION_NAME_DESKTOP)
    }

    pub fn get_keybinds_for(&self, id: &String) -> Vec<Keybind> {
        match self.keybinds.get(id) {
            Some(a) => a.to_vec(),
            None => vec![],
        }
    }

    pub fn get_profiles_for(&self, id: &String) -> Vec<Profile> {
        let mut profiles: Vec<Profile> = vec![];
        for prof in &self.profiles {
            if &prof.applicationId == id {
                profiles.push(prof.clone());
            }
        }
        profiles
    }

    pub fn get_icon_cache(&self) -> Option<path::PathBuf> {
        Some(self.lghub_location.join("icon_cache"))
    }

    fn get_application_index(&self, id: &String) -> Option<usize> {
        self.applications
            .iter()
            .position(|a| &a.applicationId == id)
    }
}
