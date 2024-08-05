use std::path;

use uuid::Uuid;

use crate::image::Image;
use crate::types::Application;
use crate::utils::{safe_canonicalize, APPLICATION_NAME_DESKTOP};
use crate::Game;

use super::{Commit, LogitechSettings};

impl LogitechSettings {
    pub fn app_from_game(&self, game: Game) -> Option<Application> {
        let apps = self.get_applications();
        Some(
            apps.iter()
                .find(|app| app.applicationId == game.id.to_string())?
                .clone(),
        )
    }

    pub fn create_application(&self, executable: &path::Path) -> Option<Application> {
        let application = self.new_application(executable);
        let profiles = self.get_profiles_for(&application);
        if profiles.is_empty() {
            // If this is empty, it means that we don't already have an existing application.
            let default = self.new_default_profile(&application);
            self.commit(application.clone());
            self.commit(default);
            return Some(application);
        }
        None
    }

    pub fn new_application(&self, executable: &path::Path) -> Application {
        let app_id = Uuid::new_v4().to_string().replace("-", "");
        let mut image = Image::from(executable);
        let icon = image.save();

        Application::custom(
            executable
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            app_id,
            safe_canonicalize(executable),
            safe_canonicalize(&icon),
        )
    }

    pub fn get_applications(&self) -> Vec<Application> {
        let settings = self.get_settings().unwrap();
        settings.applications.applications
    }

    pub fn get_application(&self, executable: &String) -> Option<Application> {
        let apps = self.get_applications();
        Some(
            apps.iter()
                .find(|item| {
                    if let Some(app_folder) = &item.applicationFolder {
                        return app_folder == executable;
                    }
                    false
                })?
                .clone(),
        )
    }

    pub fn get_desktop_application(&self) -> Application {
        let apps = self.get_applications();
        apps.iter()
            .find(|item| item.name == APPLICATION_NAME_DESKTOP && item.applicationPath.is_none())
            .expect("no desktop application")
            .clone()
    }

    pub fn update_application(&self, application: &Application) -> Option<Vec<Application>> {
        let mut apps = self.get_applications();
        let idx = apps
            .iter()
            .position(|app| app.applicationId == application.applicationId)?;
        apps.remove(idx);
        apps.insert(idx, application.clone());
        Some(apps)
    }

    pub fn remove_application(&self, application: Application) -> Option<Vec<Application>> {
        let mut apps = self.get_applications();
        let idx = apps
            .iter()
            .position(|app| app.applicationId == application.applicationId)?;
        apps.remove(idx);
        Some(apps)
    }
}
