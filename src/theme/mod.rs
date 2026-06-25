mod axis;
mod button;
mod checkbox;
mod combo_box;
mod container;
mod float;
mod menu;
mod pane_grid;
mod pick_list;
mod progress_bar;
mod radio;
mod rule;
mod scrollable;
mod slider;
mod text;
mod text_editor;
mod text_input;
mod toggler;

pub use button::ButtonClass;
pub use container::ContainerClass;

use iced::{
    Border, Color, Shadow, Vector,
    border::Radius,
    color,
    theme::{Base, Mode, Palette, Style},
};

pub const NO_BORDER: Border = Border {
    width: 0.0,
    color: Color::TRANSPARENT,
    radius: Radius {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    },
};

pub const NO_SHADOW: Shadow = Shadow {
    color: Color::TRANSPARENT,
    offset: Vector::ZERO,
    blur_radius: 0.0,
};

#[derive(Debug, Clone)]
pub struct Colors {
    // Backgrounds
    abyss: Color,
    background: Color,
    elevated: Color,

    // Foregrounds
    primary: Color,
    highlight: Color,
    success: Color,
    warning: Color,
    danger: Color,
    disabled: Color,

    // Text and lines
    text: Color,
    muted: Color,
    faint: Color,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MakoTheme {
    #[default]
    Mako,
    //Rocket,
}

impl MakoTheme {
    pub fn colors(&self) -> Colors {
        match self {
            Self::Mako => Colors {
                abyss: color!(0x000000),
                background: color!(0x000d18),
                elevated: color!(0x051622),

                primary: color!(0x49c1ad),
                highlight: color!(0x96dcb5),
                success: color!(0x96dcb5),
                warning: color!(0xcca35d),
                danger: color!(0xe38ca7),
                disabled: color!(0xaaaaaa),

                text: color!(0xdef4e4),
                muted: color!(0x3479a2),
                faint: color!(0x023d5a),
            },
        }
    }
}

impl Base for MakoTheme {
    fn default(_preference: Mode) -> Self {
        Default::default()
    }

    fn mode(&self) -> Mode {
        Mode::Dark
    }

    fn base(&self) -> Style {
        let colors = self.colors();
        match self {
            Self::Mako => Style {
                background_color: colors.abyss,
                text_color: colors.text,
            },
        }
    }

    fn palette(&self) -> Option<Palette> {
        let colors = self.colors();
        match self {
            Self::Mako => Some(Palette {
                background: colors.background,
                text: colors.text,
                primary: colors.primary,
                success: colors.success,
                warning: colors.warning,
                danger: colors.danger,
            }),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Mako => "Mako",
            //Self::Rocket => "Rocket",
        }
    }
}

impl std::fmt::Display for MakoTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Mako => write!(f, "Mako"),
        }
    }
}
