use crate::types::{Application, Profile};
use crate::utils::{MessageBox, PROFILE_NAME_DEFAULT};

use super::LogitechSettings;

impl LogitechSettings {
    pub fn get_profiles(&self) -> Vec<Profile> {
        let settings = self.get_settings();
        settings.profiles.profiles
    }

    pub fn get_profiles_for(&self, app: &Application) -> Vec<Profile> {
        let profiles = self.get_profiles();
        let mut matches = Vec::new();
        for profile in profiles {
            if profile.applicationId == app.applicationId {
                matches.push(profile.clone());
            }
        }
        matches
    }

    pub fn get_default_profile(&self, profiles: Vec<Profile>) -> Profile {
        match profiles
            .iter()
            .find(|item| item.name == PROFILE_NAME_DEFAULT)
        {
            Some(profile) => return profile.clone(),
            None => {
                MessageBox::from("Application has no default profile or it could not be located.")
                    .error();
                panic!();
            }
        }
    }

    pub fn update_profile(&self, profile: &Profile) -> Option<Vec<Profile>> {
        let mut profiles = self.get_profiles();
        let idx = profiles.iter().position(|prof| prof.id == profile.id)?;
        profiles.remove(idx);
        profiles.insert(idx, profile.clone());
        Some(profiles)
    }

    pub fn remove_profiles(&self, application: &Application) -> Vec<Profile> {
        let profiles = self.get_profiles();
        let mut new_profiles = Vec::new();
        for profile in profiles {
            if profile.applicationId != application.applicationId {
                new_profiles.push(profile);
            }
        }
        new_profiles
    }
}
