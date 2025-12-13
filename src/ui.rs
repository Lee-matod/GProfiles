use std::{cmp::Ordering, path};

use image::{ImageReader, imageops::FilterType::CatmullRom};
use slint::{ComponentHandle, Model, ModelExt, ModelRc, SharedString, VecModel};
use uuid::Uuid;

use crate::{
    App, ApplicationModel, ProfileModel, Singleton,
    config::get_config,
    types::utils::Component,
    utils::{APPLICATION_NAME_DESKTOP, file_picker},
};

impl App {
    pub fn set_callbacks(&self) {
        let singleton = self.global::<Singleton>();

        let weak = self.as_weak();
        singleton.on_select_application({
            let weak = weak.clone();
            move |application| weak.unwrap().on_select_application(application)
        });
        singleton.on_change_name({
            let weak = weak.clone();
            move |name| weak.unwrap().on_change_name(name.into())
        });
        singleton.on_change_image({
            let weak = weak.clone();
            move || weak.unwrap().on_change_image()
        });
        singleton.on_change_executable({
            let weak = weak.clone();
            move || weak.unwrap().on_change_executable()
        });
    }

    fn on_select_application(&self, model: ApplicationModel) {
        let singleton = self.global::<Singleton>();
        singleton.sync_active_application(&model.id.to_string());
    }

    fn on_change_name(&self, name: String) {
        let singleton = self.global::<Singleton>();
        let id = singleton.get_active_application_id().to_string();
        if id.is_empty() {
            return;
        }
        let mut config = get_config().write().unwrap();
        config.edit_application(&id, |mut a| {
            a.name = name.clone();
            a
        });
        drop(config);
        singleton.sync_applications();
        singleton.sync_active_application(&id);
    }

    fn on_change_image(&self) {
        let singleton = self.global::<Singleton>();
        let id = singleton.get_active_application_id().to_string();
        let mut config = get_config().write().unwrap();
        let maybe_icon_cache = config.get_icon_cache();
        config.edit_application(&id, |mut app| {
            if app.isCustom.is_none() {
                return app;
            }
            if let Some(dir) = app.posterPath.clone()
                && let Some(fp) = file_picker(
                    "Image",
                    &["bmp", "png", "jpg", "jpeg"],
                    Some(path::Path::new(&dir)),
                )
            {
                let dynimg = ImageReader::open(fp)
                    .unwrap()
                    .with_guessed_format()
                    .unwrap()
                    .decode()
                    .unwrap()
                    .resize(256, 256, CatmullRom);
                if let Some(icon_cache) = &maybe_icon_cache {
                    let f = icon_cache.join(Uuid::new_v4().to_string() + ".bmp");
                    dynimg
                        .save_with_format(&f, image::ImageFormat::Bmp)
                        .unwrap();
                    app.posterPath = Some(f.to_string_lossy().to_string());
                }
            }
            singleton.sync_application_details(&app.as_component());
            app
        });
        drop(config);
        singleton.sync_applications();
    }

    fn on_change_executable(&self) {
        let singleton = self.global::<Singleton>();
        let id = singleton.get_active_application_id().to_string();
        let mut config = get_config().write().unwrap();
        config.edit_application(&id, |mut app| {
            if app.isCustom.is_none() {
                return app;
            }
            if let Some(dir) = app.applicationPath.clone()
                && let Some(fp) = file_picker("Executable", &["exe"], Some(path::Path::new(&dir)))
            {
                app.applicationPath = Some(fp.to_string_lossy().to_string());
            }
            singleton.sync_application_details(&app.as_component());
            app
        });
    }
}

impl Singleton<'_> {
    pub fn sync(&self) {
        self.sync_applications();
        self.sync_active_application(&String::new());
    }

    pub fn sync_active_application(&self, id: &String) {
        let active = if id.is_empty() {
            self.get_desktop_application()
        } else {
            let config = get_config().read().unwrap();
            config
                .get_application(id)
                .and_then(|a| Some(a.as_component()))
        };
        if active.is_none() {
            return;
        }
        let app = active.unwrap();
        self.sync_application_details(&app);
        self.sync_keybinds_for(&app);
        self.sync_profiles_for(&app);
        self.set_active_application_id(app.id);
        self.set_active_application_type(app.r#type);
    }

    fn sync_application_details(&self, application: &ApplicationModel) {
        self.set_name(application.name.clone());
        self.set_image(application.image_path.clone());
        self.set_executable(application.executable.clone());
    }

    pub fn sync_keybinds_for(&self, application: &ApplicationModel) {
        let config = get_config().read().unwrap();
        self.set_keybinds(
            config
                .get_keybinds_for(&application.id.to_string())
                .as_component(),
        );
    }

    pub fn sync_profiles_for(&self, application: &ApplicationModel) {
        let config = get_config().read().unwrap();
        let profiles = config.get_profiles_for(&application.id.to_string());
        let items: ModelRc<ProfileModel> = profiles.as_component();

        if let Some(active) = items.iter().find(|p| p.active).or(items.row_data(0)) {
            self.set_profile_name(active.name.clone());
            self.set_profile_id(active.id.clone());
            self.set_active_profile_id(active.id.clone());
        }
        let display_name = if !application.display_name.is_empty() {
            application.display_name.clone()
        } else {
            application.name.clone()
        };
        self.set_profile_app_name(SharedString::from(display_name));
        self.set_profile_app_id(SharedString::from(application.id.clone()));
        self.set_profiles(items);
    }

    pub fn sync_applications(&self) {
        let config = get_config().read().unwrap();
        let mut items: Vec<ApplicationModel> = config.get_applications().as_component();
        items.sort_by(|a, b| {
            if a.name == APPLICATION_NAME_DESKTOP {
                Ordering::Less
            } else if b.name == APPLICATION_NAME_DESKTOP {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        self.set_applications(ModelRc::new(VecModel::from(items)));
    }

    // pub fn sync_advanced(&self) {}

    // pub fn sync_processes(&self) {}

    fn get_desktop_application(&self) -> Option<ApplicationModel> {
        self.get_applications()
            .filter(|a| a.name == APPLICATION_NAME_DESKTOP)
            .row_data(0)
    }
}
