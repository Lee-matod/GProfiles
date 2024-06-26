// Allow binary to be called GProfiles
#![allow(non_snake_case)]
// Hide terminal window
#![windows_subsystem = "windows"]

mod callbacks;
mod extract;
mod handler;
mod interface;
mod load;
mod types;

use interface::Interface;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let interface = Interface::new(ui);
    interface.start()?;
    Ok(())
}
