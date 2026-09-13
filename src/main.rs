// Hide console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use timescape_viewer::launch;

pub fn main() -> iced::Result {
    launch()
}
