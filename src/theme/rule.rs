use iced::border::Radius;
use iced::widget::rule::{Catalog, FillMode, Style};

use super::MakoTheme;

#[derive(Default)]
pub enum RuleClass {
    #[default]
    Standard,
}

impl RuleClass {
    fn style(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        Style {
            color: colors.abyss,
            radius: Radius::new(0),
            fill_mode: FillMode::Padded(2),
            snap: true,
        }
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = RuleClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.style(self)
    }
}
