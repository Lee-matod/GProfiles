// Allow binary to be called GProfiles
#![allow(non_snake_case)]
// Hide terminal window
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod types;
mod ui;
mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;

    let singleton = ui.global::<Singleton>();
    singleton.sync();
    ui.set_callbacks();

    ui.run()?;
    Ok(())
}
