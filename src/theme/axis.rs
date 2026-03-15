use crate::widgets::axis::{Catalog, Style, StyleFn};

use super::MakoTheme;

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default style of an [`crate::widgets::Axis`].
pub fn default(theme: &MakoTheme) -> Style {
    let colors = theme.colors();
    Style {
        lines: colors.faint,
        labels: colors.muted,
    }
}
