use iced::widget::button::{Catalog, Status, Style};

use super::{MakoTheme, NO_BORDER, NO_SHADOW};

#[derive(Default)]
pub enum ButtonClass {
    #[default]
    Standard,
    Secondary,
    Danger,
}

impl ButtonClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let (background, text) = match self {
            ButtonClass::Standard => (colors.primary, colors.background),
            ButtonClass::Secondary => (colors.background, colors.primary),
            ButtonClass::Danger => (colors.background, colors.danger),
        };
        Style {
            background: Some(background.into()),
            text_color: text,
            border: NO_BORDER,
            shadow: NO_SHADOW,
            snap: true,
        }
    }

    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let background = match self {
            ButtonClass::Standard => colors.highlight,
            _ => colors.elevated,
        };
        self.active(style).with_background(background)
    }

    fn disabled(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let (background, text) = match self {
            ButtonClass::Standard => (colors.disabled, colors.background),
            ButtonClass::Secondary => (colors.background, colors.disabled),
            ButtonClass::Danger => (colors.background, colors.disabled),
        };
        Style {
            background: Some(background.into()),
            text_color: text,
            border: NO_BORDER,
            shadow: NO_SHADOW,
            snap: true,
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active | Status::Pressed => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
        }
    }
}
