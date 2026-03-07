use iced::border::{Border, Radius};
use iced::widget::progress_bar::{Catalog, Style};

use super::MakoTheme;

#[derive(Default)]
pub enum ButtonClass {
    #[default]
    Standard,
}

impl ButtonClass {
    fn style(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: colors.abyss.into(),
            bar: colors.primary.into(),
            border: Border {
                color: colors.elevated,
                width: 1.0,
                radius: Radius::new(0),
            },
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.style(self)
    }
}
