use iced::widget::float::{Catalog, Style};

use super::{MakoTheme, NO_SHADOW};

#[derive(Default)]
pub enum FloatClass {
    #[default]
    Standard,
}

impl FloatClass {
    fn style(&self, _style: &MakoTheme) -> Style {
        Style {
            shadow: NO_SHADOW,
            shadow_border_radius: 0.0.into(),
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = FloatClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.style(self)
    }
}
