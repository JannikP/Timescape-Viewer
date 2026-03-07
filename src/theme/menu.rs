use iced::overlay::menu::{Catalog, Style};

use super::{MakoTheme, NO_BORDER, NO_SHADOW};

#[derive(Default)]
pub enum MenuClass {
    #[default]
    Standard,
}

impl MenuClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: colors.elevated.into(),
            border: NO_BORDER,
            text_color: colors.text,
            selected_text_color: colors.background,
            selected_background: colors.primary.into(),
            shadow: NO_SHADOW,
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = MenuClass;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        MenuClass::default()
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class.active(self)
    }
}
