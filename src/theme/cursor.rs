use crate::theme::MakoTheme;
use crate::widgets::cursor::{Catalog, Style, StyleFn};

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default style of a [`crate::widgets::Cursors`] overlay.
pub fn default(theme: &MakoTheme) -> Style {
    let colors = theme.colors();
    Style {
        primary: colors.primary,
        secondary: colors.highlight,
    }
}
