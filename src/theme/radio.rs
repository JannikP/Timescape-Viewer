use iced::widget::radio::{Catalog, Status, Style};

use super::MakoTheme;

#[derive(Default)]
pub enum RadioClass {
    #[default]
    Standard,
}

impl RadioClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            background: colors.primary.into(),
            dot_color: colors.background,
            border_width: 3.0,
            border_color: colors.primary,
            text_color: Some(colors.text),
        }
    }

    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.highlight.into();
        style.border_color = colors.highlight;
        style.dot_color = colors.elevated;
        style
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = RadioClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active { is_selected: _ } => class.active(self),
            Status::Hovered { is_selected: _ } => class.hovered(self),
        }
    }
}
