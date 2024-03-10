use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_icon("./assets/app.ico");
        res.compile().unwrap();
    }
    slint_build::compile("ui/appwindow.slint").unwrap();
}
