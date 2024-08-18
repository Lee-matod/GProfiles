use std::path;

use rusqlite::Connection;

use crate::types::{Application, InnerApplications, InnerProfiles, JsonData, Profile};
use crate::utils::{handle_err, logitech_folder, MessageBox};

mod applications;
mod keymapper;
mod profiles;

pub trait Commit<T> {
    fn commit(&self, settings: T) -> ();
}

pub struct LogitechSettings {
    conn: Connection,
}

impl LogitechSettings {
    pub fn create() -> () {
        let database = LogitechSettings::database();
        let conn = Connection::open(database).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keymapper (
                    idx INTEGER PRIMARY KEY,
                    ptr INTEGER NOT NULL,
                    obj INTEGER NOT NULL,
                    exe TEXT NOT NULL
                );",
            (),
        )
        .unwrap();
        conn.close().unwrap();
    }

    pub fn new() -> LogitechSettings {
        let database = LogitechSettings::database();
        LogitechSettings {
            conn: Connection::open(database).unwrap(),
        }
    }

    pub fn database() -> path::PathBuf {
        let ghub = logitech_folder();
        let settings = ghub.join("settings.db");
        if !settings.exists() {
            MessageBox::new(
                "settings.db does not exist.",
                "%LOCALAPPDATA%\\LGHUB\\settings.db not found.",
            )
            .error();
            panic!();
        }
        settings
    }

    pub fn get_settings(&self) -> JsonData {
        let decoded = self.get_raw_settings();
        match serde_json::from_str(&decoded) {
            Ok(data) => data,
            Err(err) => {
                MessageBox::from_error("Could not decode JSON.", err.to_string()).error();
                panic!();
            }
        }
    }

    pub fn close(self) -> () {
        self.conn.close().unwrap();
    }

    fn get_raw_settings(&self) -> String {
        let mut stmt = self.conn.prepare("SELECT file FROM data;").unwrap();
        let row: Vec<u8> = stmt.query_row([], |row| Ok(row.get(0)?)).unwrap();
        String::from_utf8_lossy(&row).to_string()
    }
}

impl Commit<Vec<Application>> for LogitechSettings {
    fn commit(&self, settings: Vec<Application>) -> () {
        let decoded = self.get_raw_settings();
        let mut raw: serde_json::Value = match serde_json::from_str(&decoded) {
            Ok(data) => data,
            Err(err) => return handle_err("Could not decode JSON.", Box::new(err)),
        };
        raw["applications"]["applications"] = InnerApplications {
            applications: settings,
        }
        .into();
        self.conn
            .execute(
                "UPDATE data SET file=?;",
                (serde_json::to_string(&raw).unwrap().as_bytes(),),
            )
            .unwrap();
    }
}

impl Commit<Vec<Profile>> for LogitechSettings {
    fn commit(&self, settings: Vec<Profile>) -> () {
        let decoded = self.get_raw_settings();
        let mut raw: serde_json::Value = match serde_json::from_str(&decoded) {
            Ok(data) => data,
            Err(err) => return handle_err("Could not decode JSON.", Box::new(err)),
        };
        raw["profiles"]["profiles"] = InnerProfiles { profiles: settings }.into();
        self.conn
            .execute(
                "UPDATE data SET file=?;",
                (serde_json::to_string(&raw).unwrap().as_bytes(),),
            )
            .unwrap();
    }
}

impl Commit<Application> for LogitechSettings {
    fn commit(&self, settings: Application) -> () {
        let applications = self.get_applications();

        let decoded = self.get_raw_settings();
        let mut raw: serde_json::Value = match serde_json::from_str(&decoded) {
            Ok(data) => data,
            Err(err) => return handle_err("Could not decode JSON.", Box::new(err)),
        };

        match applications
            .iter()
            .find(|app| app.applicationId == settings.applicationId)
        {
            Some(_) => {
                let new_apps = self.update_application(&settings).unwrap();
                self.commit(new_apps);
            }
            None => {
                raw["applications"]["applications"]
                    .as_array_mut()
                    .unwrap()
                    .push(settings.into());
                self.conn
                    .execute(
                        "UPDATE data SET file=?;",
                        (serde_json::to_string(&raw).unwrap().as_bytes(),),
                    )
                    .unwrap();
            }
        };
    }
}

impl Commit<Profile> for LogitechSettings {
    fn commit(&self, settings: Profile) -> () {
        let profiles = self.get_profiles();

        let decoded = self.get_raw_settings();
        let mut raw: serde_json::Value = match serde_json::from_str(&decoded) {
            Ok(data) => data,
            Err(err) => return handle_err("Could not decode JSON.", Box::new(err)),
        };

        match profiles.iter().find(|profile| profile.id == settings.id) {
            Some(_) => {
                let new_profs = self.update_profile(&settings).unwrap();
                self.commit(new_profs);
            }
            None => {
                raw["profiles"]["profiles"]
                    .as_array_mut()
                    .unwrap()
                    .push(settings.into());
                self.conn
                    .execute(
                        "UPDATE data SET file=?;",
                        (serde_json::to_string(&raw).unwrap().as_bytes(),),
                    )
                    .unwrap();
            }
        };
    }
}
