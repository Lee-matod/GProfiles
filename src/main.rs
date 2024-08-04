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

use callbacks::Callbacks;
use image::Image;
use utils::safe_canonicalize;
use uuid::Uuid;

use crate::settings::LogitechSettings;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    LogitechSettings::create();
    let ui = AppWindow::new()?;

    ui.on_application_clicked({
        let weak = ui.as_weak();
        move |application| Callbacks::application_clicked(weak.unwrap(), application)
    });

    ui.on_restart_ghub(move || Callbacks::restart_ghub());

    ui.on_from_executable({
        let weak = ui.as_weak();
        move || Callbacks::from_executable(weak.unwrap())
    });

    ui.on_from_process({
        let weak = ui.as_weak();
        move |process| Callbacks::from_process(weak.unwrap(), process)
    });

    ui.on_name_edit({
        let weak = ui.as_weak();
        move || Callbacks::name_edit(weak.unwrap())
    });

    ui.on_image_edit({
        let weak = ui.as_weak();
        move || {
            Callbacks::file_edit(
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
        }
    });

    ui.on_executable_edit({
        let weak = ui.as_weak();
        move || {
            Callbacks::file_edit(
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
        }
    });

    ui.on_forget_application({
        let weak = ui.as_weak();
        move || Callbacks::forget_application(weak.unwrap())
    });

    ui.on_new_key({
        let weak = ui.as_weak();
        move || Callbacks::new_key(weak.unwrap())
    });

    ui.on_set_pointer({
        let weak = ui.as_weak();
        move |keybind| Callbacks::set_pointer(weak.unwrap(), keybind)
    });

    ui.on_set_object({
        let weak = ui.as_weak();
        move |keybind| Callbacks::set_object(weak.unwrap(), keybind)
    });

    ui.on_delete_key({
        let weak = ui.as_weak();
        move |keybind| Callbacks::delete_key(weak.unwrap(), keybind)
    });

    ui.start();
    Ok(())
}
