use std::path;

use rusqlite::Connection;

use crate::types::{Application, InnerApplications, InnerProfiles, JsonData, Profile};
use crate::utils::logitech_folder;

mod applications;
mod keymapper;
mod profiles;

pub trait Commit<T> {
    fn commit(&self, settings: T) -> Option<()>;
}

pub struct LogitechSettings {
    conn: Connection,
}

impl LogitechSettings {
    pub fn create() -> Option<()> {
        let database = LogitechSettings::database();
        let conn = Connection::open(database).ok()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keymapper (
                    idx INTEGER PRIMARY KEY,
                    ptr INTEGER NOT NULL,
                    obj INTEGER NOT NULL,
                    exe TEXT NOT NULL
                );",
            (),
        )
        .ok()?;
        conn.close().ok()?;
        Some(())
    }

    pub fn new() -> Option<LogitechSettings> {
        let database = LogitechSettings::database();
        Some(LogitechSettings {
            conn: Connection::open(database).ok()?,
        })
    }

    pub fn database() -> path::PathBuf {
        let ghub = logitech_folder();
        let settings = ghub.join("settings.db");
        if !settings.exists() {
            panic!("settings.db not in LGHUB");
        }
        settings
    }

    pub fn get_settings(&self) -> Option<JsonData> {
        let decoded = self.get_raw_settings()?;
        serde_json::from_str(&decoded).ok()
    }

    pub fn close(self) -> Option<()> {
        self.conn.close().ok()?;
        Some(())
    }

    fn get_raw_settings(&self) -> Option<String> {
        let mut stmt = self.conn.prepare("SELECT file FROM data;").ok()?;
        let row: Vec<u8> = stmt.query_row([], |row| Ok(row.get(0)?)).ok()?;
        Some(String::from_utf8_lossy(&row).to_string())
    }
}

impl Commit<Vec<Application>> for LogitechSettings {
    fn commit(&self, settings: Vec<Application>) -> Option<()> {
        let decoded = self.get_raw_settings()?;
        let mut raw: serde_json::Value = serde_json::from_str(&decoded).ok()?;
        raw["applications"]["applications"] = InnerApplications {
            applications: settings,
        }
        .into();
        self.conn
            .execute(
                "UPDATE data SET file=?;",
                (serde_json::to_string(&raw).ok()?.as_bytes(),),
            )
            .ok()?;
        Some(())
    }
}

impl Commit<Vec<Profile>> for LogitechSettings {
    fn commit(&self, settings: Vec<Profile>) -> Option<()> {
        let decoded = self.get_raw_settings()?;
        let mut raw: serde_json::Value = serde_json::from_str(&decoded).ok()?;
        raw["profiles"]["profiles"] = InnerProfiles { profiles: settings }.into();
        self.conn
            .execute(
                "UPDATE data SET file=?;",
                (serde_json::to_string(&raw).ok()?.as_bytes(),),
            )
            .ok()?;
        Some(())
    }
}

impl Commit<Application> for LogitechSettings {
    fn commit(&self, settings: Application) -> Option<()> {
        let applications = self.get_applications();

        let decoded = self.get_raw_settings()?;
        let mut raw: serde_json::Value = serde_json::from_str(&decoded).ok()?;

        match applications
            .iter()
            .find(|app| app.applicationId == settings.applicationId)
        {
            Some(_) => {
                let new_apps = self.update_application(&settings)?;
                self.commit(new_apps)?;
            }
            None => {
                raw["applications"]["applications"]
                    .as_array_mut()?
                    .push(settings.into());
                self.conn
                    .execute(
                        "UPDATE data SET file=?;",
                        (serde_json::to_string(&raw).ok()?.as_bytes(),),
                    )
                    .ok()?;
            }
        };
        Some(())
    }
}

impl Commit<Profile> for LogitechSettings {
    fn commit(&self, settings: Profile) -> Option<()> {
        let profiles = self.get_profiles();

        let decoded = self.get_raw_settings()?;
        let mut raw: serde_json::Value = serde_json::from_str(&decoded).ok()?;

        match profiles.iter().find(|profile| profile.id == settings.id) {
            Some(_) => {
                let new_profs = self.update_profile(&settings)?;
                self.commit(new_profs)?;
            }
            None => {
                raw["profiles"]["profiles"]
                    .as_array_mut()?
                    .push(settings.into());
                self.conn
                    .execute(
                        "UPDATE data SET file=?;",
                        (serde_json::to_string(&raw).ok()?.as_bytes(),),
                    )
                    .ok()?;
            }
        };
        Some(())
    }
}
