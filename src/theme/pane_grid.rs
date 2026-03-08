use iced::widget::pane_grid::{Catalog, Highlight, Line, Style, StyleFn};

use crate::constants::layout::PANEL_GAP;

use super::{MakoTheme, NO_BORDER};

/// The default style of a [`iced::widget::PaneGrid`].
pub fn default(style: &MakoTheme) -> Style {
    let colors = style.colors();
    Style {
        hovered_region: Highlight {
            background: colors.elevated.into(),
            border: NO_BORDER,
        },
        picked_split: Line {
            color: colors.primary,
            width: PANEL_GAP as f32,
        },
        hovered_split: Line {
            color: colors.highlight,
            width: PANEL_GAP as f32,
        },
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}
