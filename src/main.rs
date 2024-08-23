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
    delete_key, forget_application, new_application, new_key, property_edit, restart_ghub, set_key,
};
use extract::get_lock;
use image::Image;
use slint::SharedString;
use utils::{safe_canonicalize, MessageBox};
use uuid::Uuid;

use crate::settings::LogitechSettings;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mutex = unsafe { get_lock() };
    if mutex.is_none() {
        MessageBox::from(
            "Application is already running. Exit the application through the system tray to start a new instance."
        ).info();
        return Ok(());
    }

    LogitechSettings::create();
    let ui = AppWindow::new()?;

    // I wish there were a nicer way to do this.

    ui.on_application_clicked({
        let weak = ui.as_weak();
        move |application| weak.unwrap().set_active(application)
    });

    ui.on_restart_ghub(move || restart_ghub());

    ui.on_from_executable({
        let weak = ui.as_weak();
        move || {
            let app = weak.unwrap();
            let executable = match app.select_file("Executable", &["exe"], path::Path::new("")) {
                Some(i) => i,
                None => return, // The file dialog was closed.
            };
            new_application(app, executable);
        }
    });

    ui.on_from_process({
        let weak = ui.as_weak();
        move |process| {
            new_application(weak.unwrap(), process.executable.to_string());
        }
    });

    ui.on_name_edit({
        let weak = ui.as_weak();
        move || {
            property_edit(
                weak.unwrap(),
                move |app, active| {
                    active.name = app.get_active_application_name();
                    Some(active.clone())
                },
                move |app, game| {
                    app.name = game.name.to_string();
                },
            )
        }
    });

    ui.on_image_edit({
        let weak = ui.as_weak();
        move || {
            property_edit(
                weak.unwrap(),
                move |app, active| {
                    let image_path = app.select_file(
                        "Image",
                        &["png", "bmp"],
                        path::Path::new(&app.get_active_application_image().to_string()),
                    )?;
                    let obj = Image::from(path::Path::new(&image_path));
                    let bmp =
                        safe_canonicalize(&obj.with_filename(Uuid::new_v4().to_string()).save());
                    active.image_path = SharedString::from(bmp);
                    Some(active.clone())
                },
                move |app, game| {
                    app.posterPath = Some(game.image_path.to_string());
                },
            );
        }
    });

    ui.on_executable_edit({
        let weak = ui.as_weak();
        move || {
            property_edit(
                weak.unwrap(),
                |app, active| {
                    let exec_path = app.select_file(
                        "Executable",
                        &["exe"],
                        path::Path::new(&app.get_active_application_executable().to_string()),
                    )?;
                    active.executable =
                        SharedString::from(safe_canonicalize(path::Path::new(&exec_path)));
                    Some(active.clone())
                },
                move |app, game| {
                    app.applicationPath = Some(game.executable.to_string());
                },
            );
        }
    });

    ui.on_forget_application({
        let weak = ui.as_weak();
        move || forget_application(weak.unwrap())
    });

    ui.on_new_key({
        let weak = ui.as_weak();
        move || new_key(weak.unwrap())
    });

    ui.on_set_pointer({
        let weak = ui.as_weak();
        move |keybind| {
            set_key(
                weak.unwrap(),
                keybind,
                move |key| Keybind::pointer_listening(key.clone()),
                move |keybind, key| keybind.update_pointer(u64::from(&key)),
                move |key| key.input_object(),
            )
        }
    });

    ui.on_set_object({
        let weak = ui.as_weak();
        move |keybind| {
            set_key(
                weak.unwrap(),
                keybind,
                move |key| Keybind::object_listening(key.clone()),
                move |keybind, key| keybind.update_object(u64::from(&key)),
                move |key| key.input_pointer(),
            )
        }
    });

    ui.on_delete_key({
        let weak = ui.as_weak();
        move |keybind| delete_key(weak.unwrap(), keybind)
    });

    ui.start(mutex.unwrap());
    Ok(())
}
