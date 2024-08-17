// Allow binary to be called GProfiles
#![allow(non_snake_case)]
// Hide terminal window
#![windows_subsystem = "windows"]

mod callbacks;
mod extract;
mod image;
mod interface;
mod objects;
mod remapper;
mod settings;
mod types;
mod utils;

use std::path;

use callbacks::{
    application_clicked, delete_key, file_edit, forget_application, from_executable, from_process,
    name_edit, new_key, restart_ghub, set_object, set_pointer,
};
use extract::get_lock;
use image::Image;
use utils::safe_canonicalize;
use uuid::Uuid;

use crate::settings::LogitechSettings;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mutex = unsafe { get_lock() };
    if mutex.is_none() {
        // There is another instance of GProfiles already running
        return Ok(());
    }

    LogitechSettings::create();
    let ui = AppWindow::new()?;

    ui.on_application_clicked({
        let weak = ui.as_weak();
        move |application| application_clicked(weak.unwrap(), application)
    });

    ui.on_restart_ghub(move || restart_ghub());

    ui.on_from_executable({
        let weak = ui.as_weak();
        move || from_executable(weak.unwrap()).unwrap_or_default()
    });

    ui.on_from_process({
        let weak = ui.as_weak();
        move |process| from_process(weak.unwrap(), process).unwrap_or_default()
    });

    ui.on_name_edit({
        let weak = ui.as_weak();
        move || name_edit(weak.unwrap()).unwrap_or_default()
    });

    ui.on_image_edit({
        let weak = ui.as_weak();
        move || {
            file_edit(
                weak.unwrap(),
                move |app| {
                    let image_path = app.select_file(
                        "Image",
                        &["png", "bmp"],
                        path::Path::new(&app.get_active_application_image().to_string()),
                    );
                    if image_path.is_none() {
                        return None;
                    }
                    let obj = Image::from(path::Path::new(&image_path.unwrap()));
                    let bmp = obj.with_filename(Uuid::new_v4().to_string()).save();
                    Some(safe_canonicalize(&bmp))
                },
                |app, p| {
                    app.posterPath = Some(p);
                },
            )
            .unwrap_or_default();
        }
    });

    ui.on_executable_edit({
        let weak = ui.as_weak();
        move || {
            file_edit(
                weak.unwrap(),
                move |app| {
                    app.select_file(
                        "Executable",
                        &["exe"],
                        path::Path::new(&app.get_active_application_executable().to_string()),
                    )
                },
                move |app, p| {
                    app.applicationPath = Some(p);
                },
            )
            .unwrap_or_default()
        }
    });

    ui.on_forget_application({
        let weak = ui.as_weak();
        move || forget_application(weak.unwrap()).unwrap_or_default()
    });

    ui.on_new_key({
        let weak = ui.as_weak();
        move || new_key(weak.unwrap())
    });

    ui.on_set_pointer({
        let weak = ui.as_weak();
        move |keybind| set_pointer(weak.unwrap(), keybind)
    });

    ui.on_set_object({
        let weak = ui.as_weak();
        move |keybind| set_object(weak.unwrap(), keybind)
    });

    ui.on_delete_key({
        let weak = ui.as_weak();
        move |keybind| delete_key(weak.unwrap(), keybind).unwrap_or_default()
    });

    ui.start(mutex.unwrap());
    Ok(())
}
