use iced::widget::container::{Catalog, Style};

use super::{MakoTheme, NO_BORDER, NO_SHADOW};

#[derive(Default)]
pub enum ContainerClass {
    #[default]
    Standard,
    Abyss,
}

impl ContainerClass {
    fn style(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: match self {
                ContainerClass::Standard => Some(colors.background.into()),
                ContainerClass::Abyss => Some(colors.abyss.into()),
            },
            text_color: Some(colors.text),
            border: NO_BORDER,
            shadow: NO_SHADOW,
            snap: true,
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.style(self)
    }
}
