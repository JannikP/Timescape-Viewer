//! [`iced::widget::pick_list::PickList`] style

use iced::widget::pick_list::{Catalog, Status, Style};

use super::{MakoTheme, NO_BORDER};

#[derive(Default)]
pub enum PickListClass {
    #[default]
    Standard,
}

impl PickListClass {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            text_color: colors.text,
            placeholder_color: colors.muted,
            handle_color: colors.primary,
            background: colors.abyss.into(),
            border: NO_BORDER,
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let mut style = self.active(style);
        style.background = colors.elevated.into();
        style
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = PickListClass;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        <Self as Catalog>::Class::default()
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered | Status::Opened { .. } => class.hovered(self),
        }
    }
}
