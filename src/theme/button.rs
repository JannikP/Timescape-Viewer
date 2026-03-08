use iced::widget::button::{Catalog, Status, Style};

use super::{MakoTheme, NO_BORDER, NO_SHADOW};

#[derive(Default)]
pub enum ButtonClass {
    #[default]
    Standard,
}

impl ButtonClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: Some(colors.primary.into()),
            text_color: colors.background,
            border: NO_BORDER,
            shadow: NO_SHADOW,
            snap: true,
        }
    }

    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        self.active(style).with_background(colors.highlight)
    }

    fn disabled(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        self.active(style).with_background(colors.disabled)
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
