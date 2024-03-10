use std::path;

use crate::interface::Interface;
use crate::processes::resolve_path;
use crate::types::{Application, Profile};

pub fn wrapper(interface: &Interface, callback: fn(&Interface) -> Result<Vec<Application>, ()>) {
    interface.ui.invoke_dialog_lock_acquire();
    match callback(interface) {
        Ok(data) => interface.handler.commit(data, None),
        Err(_) => {}
    }
    interface.ui.invoke_dialog_lock_release();
}

pub fn on_name_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = interface.displayed_profile().unwrap();
    let mut app = data[idx].clone();

    let name = interface.ui.get_profile_field_name();
    if name == "" {
        return Err(());
    }

    app.name = name.to_string();
    data[idx] = app;
    Ok(data)
}

pub fn on_image_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = interface.displayed_profile().unwrap();
    let mut app = data[idx].clone();

    let path_selected = interface.select_file(
        "Image",
        &["png"],
        path::Path::new(&interface.ui.get_profile_field_img().to_string()),
    )?;

    let bmp_image = interface
        .handler
        .save_image_for(app.applicationId.clone(), &path_selected)
        .unwrap();
    let canon = resolve_path(bmp_image.as_path());
    interface
        .ui
        .set_profile_field_img(slint::SharedString::from(&canon));
    app.posterPath = Some(canon);
    data[idx] = app;
    Ok(data)
}

pub fn on_exec_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = interface.displayed_profile().unwrap();
    let mut app = data[idx].clone();

    let path_selected = interface.select_file(
        "Executable",
        &["exe"],
        path::Path::new(&interface.ui.get_profile_field_exec().to_string()),
    )?;
    interface
        .ui
        .set_profile_field_exec(slint::SharedString::from(&path_selected));
    app.applicationPath = Some(path_selected);
    data[idx] = app;
    Ok(data)
}

pub fn on_forget_app(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = interface.displayed_profile().unwrap();

    let app = data.remove(idx);
    let app_profiles = interface.handler.find_profiles(&app);

    let settings = interface.handler.settings().unwrap();
    let mut profiles: Vec<Profile> = Vec::new();

    for profile in settings.profiles.profiles.iter() {
        if !app_profiles.contains(profile) {
            profiles.push(profile.clone());
        }
    }

    interface.handler.commit(data, Some(profiles));
    interface.reset_fields();

    Err(())
}

pub fn on_add_app(interface: &Interface) -> Result<Vec<Application>, ()> {
    let selected = interface.select_file("Executable", &["exe"], path::Path::new(""))?;
    let as_path = path::Path::new(&selected);
    let (app, profiles) = interface.handler.create_application(as_path);
    interface.handler.commit(app, Some(profiles));
    interface.reset_fields();
    Err(())
}
