//! This module contains `&'static str` constants for the icons in the
//! FiraSansCondensed-Regular-Expanded font. The icons are encoded in the
//! private use area [U+E000...U+F8FF] of Unicode.

use iced::window::Icon;
use iced::window::icon::from_file_data;

pub const MENU_ICON: &str = "\u{0E000}";
pub const LINE_CHART_ICON: &str = "\u{0E001}";
pub const SPECTROGRAM_ICON: &str = "\u{0E002}";
pub const TRAIL_CHART_ICON: &str = "\u{0E003}";
pub const ADD_ICON: &str = "\u{0E004}";
pub const DELETE_ICON: &str = "\u{0E005}";
pub const SHOW_ICON: &str = "\u{0E006}";
pub const HIDE_ICON: &str = "\u{0E007}";
pub const COLLAPSE_DIAGONAL_ICON: &str = "\u{0E008}";
pub const COLLAPSE_HORIZONTAL_ICON: &str = "\u{0E009}";
pub const COLLAPSE_VERTICAL_ICON: &str = "\u{0E00A}";
pub const FULLSCREEN_ICON: &str = "\u{0E00B}";
pub const EXIT_FULLSCREEN_ICON: &str = "\u{0E00C}";
pub const CLOSE_ICON: &str = "\u{0E00D}";
pub const PLAY_ICON: &str = "\u{0E00E}";
pub const PAUSE_ICON: &str = "\u{0E00F}";
pub const STOP_ICON: &str = "\u{0E010}";
pub const REWIND_ICON: &str = "\u{0E011}";
pub const SPEED_ICON: &str = "\u{0E012}";
pub const SKIP_BACK_ICON: &str = "\u{0E013}";
pub const SKIP_FORWARD_ICON: &str = "\u{0E014}";
pub const REWIND_START_ICON: &str = "\u{0E015}";
pub const FORWARD_END_ICON: &str = "\u{0E016}";
pub const SIGNAL_ICON: &str = "\u{0E017}";

pub fn app_icon() -> Option<Icon> {
    Some(
        from_file_data(
            include_bytes!("../../assets/app_icon_32px.png"),
            Some(image::ImageFormat::Png),
        )
        .expect("Failed to decode app icon."),
    )
}
