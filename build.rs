#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./assets/app_icon.ico");
    // TODO: Add manifest file
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {}
