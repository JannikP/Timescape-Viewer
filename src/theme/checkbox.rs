use iced::widget::checkbox::{Catalog, Status, Style};

use super::{MakoTheme, NO_BORDER};

#[derive(Default)]
pub enum CheckboxClass {
    #[default]
    Standard,
}

impl CheckboxClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: colors.primary.into(),
            icon_color: colors.background,
            border: NO_BORDER,
            text_color: Some(colors.text),
        }
    }

    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.highlight.into();
        style
    }

    fn disabled(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.disabled.into();
        style
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = CheckboxClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active { is_checked: _ } => class.active(self),
            Status::Hovered { is_checked: _ } => class.hovered(self),
            Status::Disabled { is_checked: _ } => class.disabled(self),
        }
    }
}
