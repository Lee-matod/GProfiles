use std::path;

use crate::extract::safe_canonicalize;
use crate::interface::Interface;
use crate::load::Image;
use crate::types::{Application, Profile};
use crate::AppWindow;

/// A helper method that wraps common funcitonality for button callbacks.
///
/// Dialog locking is automatically acquired before invoking the provided callback and released afterwards.
///
/// The provided `app` should be given using `AppWindow.as_weak` and unwrapping it.
pub fn wrapper(app: AppWindow, callback: fn(&Interface) -> Result<Vec<Application>, ()>) {
    let interface = &Interface::dummy(app);
    interface.ui.invoke_dialog_lock_acquire();
    match callback(interface) {
        Ok(data) => interface.handler.commit(data, None),
        Err(_) => {}
    }
    interface.ui.invoke_dialog_lock_release();
}

/// Callback that is invoked whenever a profile name is edited through LineEdit.
pub fn on_name_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = match interface.displayed_profile() {
        Ok(ret) => ret,
        Err(_) => return Err(()),
    };
    let mut app = data[idx].clone();

    let name = interface.ui.get_profile_field_name();
    if name == "" {
        return Err(());
    }

    app.name = name.to_string();
    data[idx] = app;
    Ok(data)
}

/// Callback that is invoked whenever a profile thumbnail is changed through the Profile Editor.
pub fn on_image_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = match interface.displayed_profile() {
        Ok(ret) => ret,
        Err(_) => return Err(()),
    };
    let mut app = data[idx].clone();

    let path_selected = interface.select_file(
        "Image",
        &["png", "bmp"],
        path::Path::new(&interface.ui.get_profile_field_img().to_string()),
    )?;

    let img = Image::from_path(path::Path::new(&path_selected));
    let bmp = img.save_to_cache(app.applicationId.clone());
    let canon = safe_canonicalize(bmp.as_path());
    interface
        .ui
        .set_profile_field_img(slint::SharedString::from(&canon));
    app.posterPath = Some(canon);
    data[idx] = app;
    Ok(data)
}

/// Callback that is invoked whenever a profile executable location is changed through the Profile Editor.
pub fn on_exec_edit(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = match interface.displayed_profile() {
        Ok(ret) => ret,
        Err(_) => return Err(()),
    };
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

/// Callback that is invoked whenever a profile is deleted through the Forget button.
pub fn on_forget_app(interface: &Interface) -> Result<Vec<Application>, ()> {
    let (mut data, idx) = match interface.displayed_profile() {
        Ok(ret) => ret,
        Err(_) => return Err(()),
    };

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
    interface.handler.delete_image(app.applicationId);

    Err(()) // This is intentional. We have already commited our changes.
}

/// Callback that is invoked whenever a new profile should be added through an executable path.
///
/// This opens the File Explorer as a dialog to select the path.
pub fn on_add_app(interface: &Interface) -> Result<Vec<Application>, ()> {
    let selected = interface.select_file("Executable", &["exe"], path::Path::new(""))?;
    let as_path = path::Path::new(&selected);
    let (app, profiles) = interface.handler.create_application(as_path);
    interface.handler.commit(app, Some(profiles));
    interface.reset_fields();
    Err(())
}
