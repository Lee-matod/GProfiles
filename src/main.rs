// Allow binary to be called GProfiles
#![allow(non_snake_case)]
// Hide terminal window
#![windows_subsystem = "windows"]

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;
    ui.run()?;
    Ok(())
}
