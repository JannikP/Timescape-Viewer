use iced::Color;
use iced::widget::toggler::{Catalog, Status, Style, StyleFn};

use super::MakoTheme;

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`iced::widget::Toggler`].
pub fn default(theme: &MakoTheme, status: Status) -> Style {
    let colors = theme.colors();

    let background = match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
            if is_toggled {
                colors.primary
            } else {
                colors.elevated
            }
        }
        Status::Disabled { is_toggled } => {
            if is_toggled {
                colors.faint
            } else {
                colors.abyss
            }
        }
    };

    let foreground = match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                colors.primary
            } else {
                colors.elevated
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                colors.highlight
            } else {
                colors.faint
            }
        }
        Status::Disabled { .. } => colors.disabled,
    };

    Style {
        background: background.into(),
        foreground: foreground.into(),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        text_color: None,
        border_radius: None,
        padding_ratio: 0.1,
    }
}
