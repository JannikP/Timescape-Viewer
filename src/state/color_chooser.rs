use iced::Color;

use crate::messages::{Message, line_chart::LineChartMessage};
use crate::state::line_chart::{LineChartLegendEntry, LineColor};
use crate::theme::MakoTheme;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct ColorChooser {
    /// Index of the line chart.
    chart: usize,

    /// Index of the edited signal.
    entry: usize,

    /// Authority selection: Let the [Theme](crate::MakoTheme) decide the color or take the custom,
    /// user defined color.
    authority: Authority,

    /// Intermediate state of the user's color choice.
    user_color: Color,
}

impl ColorChooser {
    pub fn from(chart: usize, entry: &LineChartLegendEntry, theme: &MakoTheme) -> Self {
        ColorChooser {
            chart,
            entry: entry.index,
            authority: entry.color.into(),
            user_color: entry.color.color(theme, entry.index),
        }
    }

    pub fn choice(&self) -> LineColor {
        match self.authority {
            Authority::Automatic => LineColor::Automatic,
            Authority::UserDefined => LineColor::UserDefined(self.user_color),
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
