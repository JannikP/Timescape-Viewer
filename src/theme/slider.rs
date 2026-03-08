use iced::Color;
use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};

use super::{MakoTheme, NO_BORDER};

/// The default style of a [`iced::widget::Slider`].
pub fn default(theme: &MakoTheme, status: Status) -> Style {
    let colors = theme.colors();

    let color = match status {
        Status::Active => colors.primary,
        Status::Hovered => colors.highlight,
        Status::Dragged => colors.highlight,
    };

    Style {
        rail: Rail {
            backgrounds: (color.into(), colors.elevated.into()),
            width: 4.0,
            border: NO_BORDER,
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 8.0 },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}
