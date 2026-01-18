//! This module contains `&'static str` constants for the icons in the
//! FiraSansCondensed-Regular-Expanded font. The icons are encoded in the
//! private use area [U+E000...U+F8FF] of Unicode.

use iced::window::Icon;
use iced::window::icon::from_file_data;

pub const MENU_ICON: &'static str = "\u{0E000}";
pub const LINE_CHART_ICON: &'static str = "\u{0E001}";
pub const SPECTROGRAM_ICON: &'static str = "\u{0E002}";
pub const SIGNAL_ICON: &'static str = "\u{02022}";

pub fn app_icon() -> Option<Icon> {
    Some(
        from_file_data(
            include_bytes!("../../assets/app_icon_32px.png"),
            Some(image::ImageFormat::Png),
        )
        .expect("Failed to decode app icon."),
    )
}
