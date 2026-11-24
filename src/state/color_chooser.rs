use crate::messages::color_chooser::ColorChooserMessage;
use crate::messages::{Message, line_chart::LineChartMessage};
use crate::state::line_chart::{LineChartLegendEntry, LineColor};
use crate::theme::MakoTheme;
use crate::utilities::{color_to_hsv, hsv_to_color};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct ColorChooser {
    /// Index of the line chart.
    chart: usize,

    /// Index of the edited signal.
    entry: usize,

    /// Authority selection: Let the [Theme](crate::MakoTheme) decide the color or take the custom,
    /// user defined color.
    authority: Authority,

    /// Intermediate state of the user's color choice (hue)
    hue: f32,

    /// Intermediate state of the user's color choice (saturation)
    saturation: f32,

    value: f32,
}

impl ColorChooser {
    pub fn from(chart: usize, entry: &LineChartLegendEntry, theme: &MakoTheme) -> Self {
        let (h, s, v) = color_to_hsv(entry.color.color(theme, entry.index));
        ColorChooser {
            chart,
            entry: entry.index,
            authority: entry.color.into(),
            hue: h,
            saturation: s,
            value: v,
        }
    }

    pub fn authority(&self) -> Authority {
        self.authority
    }

    pub fn hue(&self) -> f32 {
        self.hue
    }

    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn choice(&self) -> LineColor {
        match self.authority {
            Authority::Automatic => LineColor::Automatic,
            Authority::UserDefined => LineColor::UserDefined(hsv_to_color(self.hue, self.saturation, self.value)),
        }
    }

    pub fn update(&mut self, message: ColorChooserMessage) {
        match message {
            ColorChooserMessage::ChangeAuthority(authority) => {
                self.authority = authority;
            }
            ColorChooserMessage::ChangeColor { hue, saturation } => {
                self.authority = Authority::UserDefined;
                self.hue = hue;
                self.saturation = saturation;
            }
        }
    }

    pub fn message(&self) -> Message {
        Message::LineChartMessage(
            self.chart,
            LineChartMessage::Color {
                signal: self.entry,
                color: self.choice(),
            },
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Authority {
    #[default]
    Automatic,

    UserDefined,
}

impl From<LineColor> for Authority {
    fn from(value: LineColor) -> Self {
        match value {
            LineColor::Automatic => Self::Automatic,
            LineColor::UserDefined(_) => Self::UserDefined,
        }
    }
}
