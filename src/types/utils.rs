use std::{io, path};

use slint::{Image, Model, ModelRc, SharedString, VecModel};

use crate::{
    ApplicationModel, ApplicationType, KeybindModel, ProfileModel,
    types::{
        gprofiles::Keybind,
        logitech::{Application, Profile},
    },
    utils::{Cast, DESKTOP_ICON, PROFILE_NAME_DEFAULT},
};

fn poster_from_name(name: &str) -> io::Result<Image> {
    let depots = path::Path::new("C:\\ProgramData\\LGHUB\\depots");
    let parent = depots.read_dir()?.last().unwrap()?;
    Ok(Image::using(
        parent.path().join("core_apps\\images").join(name),
    ))
}

fn poster_from_url(url: &String) -> reqwest::Result<Image> {
    let data = reqwest::blocking::get(url)?.bytes()?;
    Ok(Image::using(image::load_from_memory(&data).unwrap()))
}

pub trait Component<T> {
    fn as_component(&self) -> T;
}

impl Component<ApplicationModel> for Application {
    fn as_component(&self) -> ApplicationModel {
        let executable =
            SharedString::from(self.applicationPath.as_ref().unwrap_or(&String::new()));
        let id = SharedString::from(&self.applicationId);
        let name = SharedString::from(&self.name);

        let (r#type, image_path, has_icon, icon, display_name) =
            match (&self.posterPath, &self.posterUrl) {
                (Some(poster_path), _) => {
                    let as_path = path::Path::new(&poster_path);
                    (
                        ApplicationType::Installed,
                        SharedString::from(poster_path),
                        as_path.exists(),
                        Image::using(as_path),
                        SharedString::new(),
                    )
                }
                (None, Some(poster_url)) => {
                    let poster_name = poster_url.split("/").last().unwrap();
                    let as_path = match (poster_from_url(poster_url), poster_from_name(poster_name))
                    {
                        (Ok(data), _) => Some(data),
                        (Err(_), Ok(data)) => Some(data),
                        (Err(_), Err(_)) => None,
                    };
                    (
                        ApplicationType::Installed,
                        SharedString::from(poster_url),
                        as_path.is_some(),
                        as_path.unwrap_or(Image::using(path::PathBuf::new())),
                        SharedString::new(),
                    )
                }
                _ => {
                    let icon = Image::using(image::load_from_memory(DESKTOP_ICON).unwrap());
                    (
                        ApplicationType::Desktop,
                        SharedString::new(),
                        true,
                        icon,
                        SharedString::from("Desktop"),
                    )
                }
            };

        ApplicationModel {
            executable,
            icon,
            id,
            image_path,
            name,
            r#type,
            has_icon,
            display_name,
        }
    }
}

impl Component<ModelRc<ApplicationModel>> for Vec<Application> {
    fn as_component(&self) -> ModelRc<ApplicationModel> {
        ModelRc::new(
            self.iter()
                .map(|app| app.as_component())
                .collect::<VecModel<ApplicationModel>>(),
        )
    }
}

impl Component<Vec<ApplicationModel>> for Vec<Application> {
    fn as_component(&self) -> Vec<ApplicationModel> {
        Component::<ModelRc<ApplicationModel>>::as_component(self)
            .iter()
            .collect()
    }
}

impl Component<ProfileModel> for Profile {
    fn as_component(&self) -> ProfileModel {
        ProfileModel {
            id: SharedString::from(&self.id),
            name: SharedString::from(&self.name),
            active: self.activeForApplication,
            application: SharedString::from(&self.applicationId),
            display_name: if self.name == PROFILE_NAME_DEFAULT {
                SharedString::from("Default")
            } else {
                SharedString::from(&self.name)
            },
        }
    }
}

impl Component<ModelRc<ProfileModel>> for Vec<Profile> {
    fn as_component(&self) -> ModelRc<ProfileModel> {
        ModelRc::new(
            self.iter()
                .map(|prof| prof.as_component())
                .collect::<VecModel<ProfileModel>>(),
        )
    }
}

impl Component<Vec<ProfileModel>> for Vec<Profile> {
    fn as_component(&self) -> Vec<ProfileModel> {
        Component::<ModelRc<ProfileModel>>::as_component(self)
            .iter()
            .collect()
    }
}

impl Component<KeybindModel> for Keybind {
    fn as_component(&self) -> KeybindModel {
        KeybindModel {
            input: SharedString::from(&self.input),
            output: SharedString::from(&self.output),
            vk_input: self.virtual_input,
            vk_output: self.virtual_output,
        }
    }
}

impl Component<ModelRc<KeybindModel>> for Vec<Keybind> {
    fn as_component(&self) -> ModelRc<KeybindModel> {
        ModelRc::new(
            self.iter()
                .map(|key| key.as_component())
                .collect::<VecModel<KeybindModel>>(),
        )
    }
}

impl Component<Vec<KeybindModel>> for Vec<Keybind> {
    fn as_component(&self) -> Vec<KeybindModel> {
        Component::<ModelRc<KeybindModel>>::as_component(self)
            .iter()
            .collect()
    }
}
