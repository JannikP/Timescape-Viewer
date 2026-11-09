use iced::widget::container::{Catalog, Style};

use crate::theme::{MakoTheme, NO_BORDER, NO_SHADOW};

#[derive(Default)]
pub enum ContainerClass {
    Abyss,
    #[default]
    Standard,
    Elevated,
    Tinted,
}

impl ContainerClass {
    fn style(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: match self {
                ContainerClass::Abyss => Some(colors.abyss.into()),
                ContainerClass::Standard => Some(colors.background.into()),
                ContainerClass::Elevated => Some(colors.elevated.into()),
                ContainerClass::Tinted => Some(colors.background.scale_alpha(0.7).into()),
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
