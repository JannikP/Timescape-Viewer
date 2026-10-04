use iced::widget::text::{Catalog, Style, StyleFn};

use crate::theme::MakoTheme;

impl Catalog for MakoTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default text styling; color is inherited.
pub fn default(_theme: &MakoTheme) -> Style {
    Style { color: None }
}

/// Text with the default base color.
#[allow(unused)]
pub fn base(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().text),
    }
}

/// Text conveying some important information, like an action.
#[allow(unused)]
pub fn primary(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().primary),
    }
}

/// Text conveying some secondary information, like a footnote.
#[allow(unused)]
pub fn secondary(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().muted),
    }
}

/// Text conveying some positive information, like a successful event.
#[allow(unused)]
pub fn success(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().success),
    }
}

/// Text conveying some mildly negative information, like a warning.
#[allow(unused)]
pub fn warning(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().warning),
    }
}

/// Text conveying some negative information, like an error.
#[allow(unused)]
pub fn danger(theme: &MakoTheme) -> Style {
    Style {
        color: Some(theme.colors().danger),
    }
}
