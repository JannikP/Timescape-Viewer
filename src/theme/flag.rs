use crate::theme::{MakoTheme, NO_BORDER, NO_SHADOW};
use crate::widgets::flag::{Catalog, Status, Style};

#[derive(Default)]
pub enum FlagClass {
    Standard,
    #[default]
    Secondary,
    Danger,
}

impl FlagClass {
    fn active(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let (background, text) = match self {
            FlagClass::Standard => (colors.primary, colors.background),
            FlagClass::Secondary => (colors.background, colors.primary),
            FlagClass::Danger => (colors.background, colors.danger),
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
            FlagClass::Standard => colors.highlight,
            _ => colors.elevated,
        };
        self.active(style).with_background(background)
    }

    fn disabled(&self, style: &MakoTheme) -> Style {
        let colors = style.colors();
        let (background, text) = match self {
            FlagClass::Standard => (colors.disabled, colors.background),
            FlagClass::Secondary => (colors.background, colors.disabled),
            FlagClass::Danger => (colors.background, colors.disabled),
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
    type Class<'a> = FlagClass;

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
