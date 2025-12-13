use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs, path,
    sync::{OnceLock, RwLock},
};

use crate::{
    types::{
        gprofiles::{GProfilesData, Keybind},
        logitech::{Application, LogitechData, Profile},
    },
    utils::{parse_json, APPLICATION_NAME_DESKTOP},
};

pub static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

pub fn get_config() -> &'static RwLock<Config> {
    CONFIG.get_or_init(|| RwLock::new(Config::new()))
}

fn get_gprofiles_path() -> path::PathBuf {
    let parent_dir: OsString = if cfg!(target_os = "windows") {
        env::var_os("LOCALAPPDATA")
    } else if cfg!(target_os = "linux") {
        env::var_os("XDG_CONFIG_HOME")
    } else {
        None
    }
    .expect("Configuration directory not found.");

    let parent = path::Path::new(&parent_dir);
    if !parent.exists() {
        panic!("Configuration directory does not exist.");
    }

    let config_dir = parent.join("GProfiles");
    if !config_dir.exists() {
        fs::create_dir(&config_dir).expect("Failed to create configuration directory.");
    }

    let config = config_dir.join("settings.json");
    if !config.exists() {
        fs::write(&config, "{}").expect("Failed to create default configuration file.");
    }

    config
}

#[derive(Debug)]
pub struct Config {
    applications: Vec<Application>,
    profiles: Vec<Profile>,
    keybinds: HashMap<String, Vec<Keybind>>,
    gprofiles_path: path::PathBuf,
    lghub_location: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let gprofiles_path = get_gprofiles_path();
        let gprofiles_data: GProfilesData = parse_json(&gprofiles_path);

        let applications: Vec<Application>;
        let profiles: Vec<Profile>;
        let lghub_location: Option<String>;
        let keybinds = gprofiles_data.keybinds.unwrap_or_default();
        if gprofiles_data.settings.is_none() {
            applications = vec![];
            profiles = vec![];
            lghub_location = None;
        } else {
            let location = gprofiles_data.settings.unwrap();
            let logitech_data: LogitechData = parse_json(path::Path::new(&location));
            lghub_location = Some(location);
            applications = logitech_data.applications.applications;
            profiles = logitech_data.profiles.profiles;
        }

        Self {
            applications,
            profiles,
            gprofiles_path,
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

    pub fn get_lghub_path(&self) -> Option<&path::Path> {
        if let Some(s) = &self.lghub_location {
            Some(path::Path::new(s))
        } else {
            None
        }
    }

    pub fn get_icon_cache(&self) -> Option<path::PathBuf> {
        Some(self.get_lghub_path()?.join("icon_cache"))
    }

    fn get_application_index(&self, id: &String) -> Option<usize> {
        self.applications
            .iter()
            .position(|a| &a.applicationId == id)
    }
}
