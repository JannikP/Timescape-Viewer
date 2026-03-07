use iced::Border;
use iced::widget::text_input::{Catalog, Status, Style};

use super::MakoTheme;

#[derive(Default)]
pub enum TextInputClass {
    #[default]
    Standard,
}

impl TextInputClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: colors.background.into(),
            border: Border {
                color: colors.text,
                width: 1.0,
                radius: 0.into(),
            },
            icon: colors.muted,
            placeholder: colors.muted,
            value: colors.text,
            selection: colors.primary,
        }
    }

    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.elevated.into();
        style.border.color = colors.highlight;
        style
    }

    fn focused(&self, style: &MakoTheme, _is_hovered: bool) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.elevated.into();
        style.border.color = colors.highlight;
        style
    }

    fn disabled(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.abyss.into();
        style.border.color = colors.disabled;
        style
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = TextInputClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
            Status::Focused { is_hovered } => class.focused(self, is_hovered),
        }
    }
}
