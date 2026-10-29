use iced::time::Instant;
use iced::{Color, Task};
use log::info;

use crate::animation::Spring;
use crate::constants::layout::{MAXIMUM_SCOPE_HEIGHT, MINIMUM_SCOPE_HEIGHT};
use crate::core::{Domain, Trace};
use crate::messages::{Message, line_chart::LineChartMessage};
use crate::state::color_chooser::ColorChooser;
use crate::state::{Scope, ScopePlotter};
use crate::theme::MakoTheme;

#[derive(Debug, Clone)]
pub struct LineChartLegend {
    index: usize,
    signals: Vec<LineChartLegendEntry>,
    domain: Spring<Domain>,
    height: f32,
    signal_input: String,
}

impl LineChartLegend {
    pub fn push_signal<S: Into<String>>(&mut self, signal: S) {
        let index = self.signals.len();
        let entry = LineChartLegendEntry {
            index,
            signal: signal.into(),
            color: LineColor::default(),
            visible: true,
        };
        self.signals.push(entry);
    }

    pub fn remove_signal(&mut self, index: usize) {
        self.signals.remove(index);
        for (index, signal) in self.signals.iter_mut().enumerate() {
            signal.index = index
        }
    }

    pub fn iter_signals(&self) -> impl Iterator<Item = &LineChartLegendEntry> {
        self.signals.iter()
    }

    pub fn update(&mut self, message: LineChartMessage) -> Task<Message> {
        match message {
            LineChartMessage::ToggleVisibility { signal, visible } => {
                let signal = self.signals.get_mut(signal);
                if let Some(signal) = signal {
                    signal.visible = visible;
                }
            }
            LineChartMessage::InvokeColorChooser { signal } => {
                let signal = self.signals.get(signal);
                if let Some(signal) = signal {
                    let chooser = ColorChooser::from(self.index, signal, &MakoTheme::Mako);
                    return Task::done(Message::ShowModal(super::Modal::ChooseColor(chooser)));
                }
            }
            LineChartMessage::Color { signal, color } => {
                let signal = self.signals.get_mut(signal);
                if let Some(signal) = signal {
                    signal.color = color;
                }
            }
            LineChartMessage::RemoveSignal(index) => {
                self.remove_signal(index);
            }
            LineChartMessage::SignalInputChanged(input) => {
                self.signal_input = input;
            }
            LineChartMessage::SignalInputSubmit => {
                info!(
                    "Adding signal '{:}' to scope no. {:}",
                    self.signal_input.as_str(),
                    self.index
                );
                self.push_signal(self.signal_input.clone());
                self.signal_input = String::new();
            }
        }
        Task::none()
    }

    pub fn set_domain(&mut self, domain: Domain, now: Instant) {
        self.domain.go(domain, now);
    }

    pub fn signal_input(&self) -> &str {
        self.signal_input.as_str()
    }

    pub fn on_signal_input<'a>(&'a self) -> impl Fn(String) -> Message + 'a {
        |input: String| {
            Message::LineChartMessage(self.index, LineChartMessage::SignalInputChanged(input))
        }
    }

    pub fn on_signal_input_submit(&self) -> Message {
        Message::LineChartMessage(self.index, LineChartMessage::SignalInputSubmit)
    }

    pub fn domain(&self) -> &Domain {
        self.domain.value()
    }
}

impl Scope for LineChartLegend {
    fn height(&self) -> f32 {
        self.height
    }

    fn resize(&mut self, height: f32) {
        self.height = height.clamp(MINIMUM_SCOPE_HEIGHT, MAXIMUM_SCOPE_HEIGHT);
    }

    fn create_plotter(&self) -> ScopePlotter {
        ScopePlotter::LineChart(LineChartPlotter {
            signals: Vec::new(),
        })
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn is_animating(&self) -> bool {
        self.domain.is_animating()
    }

    fn update(&mut self, now: Instant) {
        self.domain.update(now);
    }
}

impl Default for LineChartLegend {
    fn default() -> Self {
        Self {
            index: 0,
            signals: Vec::new(),
            domain: Spring::smooth(Domain::default()),
            height: 150.0,
            signal_input: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineChartLegendEntry {
    pub signal: String,
    pub color: LineColor,
    pub visible: bool,
    pub index: usize,
}

impl LineChartLegendEntry {
    pub fn invoke_color_chooser_message(&self, scope: &LineChartLegend) -> Message {
        Message::LineChartMessage(
            scope.index(),
            LineChartMessage::InvokeColorChooser { signal: self.index },
        )
    }

    pub fn toggle_visibility_message(&self, scope: &LineChartLegend) -> Message {
        Message::LineChartMessage(
            scope.index(),
            LineChartMessage::ToggleVisibility {
                signal: self.index,
                visible: !self.visible,
            },
        )
    }

    pub fn delete_message(&self, scope: &LineChartLegend) -> Message {
        Message::LineChartMessage(scope.index(), LineChartMessage::RemoveSignal(self.index))
    }
}

#[derive(Debug, Clone)]
pub struct LineChartPlotter {
    signals: Vec<Trace>,
}

impl LineChartPlotter {
    pub fn iter_lines(&self) -> impl Iterator<Item = &Trace> {
        self.signals.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LineColor {
    #[default]
    Automatic,

    UserDefined(Color),
}

impl LineColor {
    pub fn color(&self, theme: &MakoTheme, index: usize) -> Color {
        match self {
            Self::Automatic => theme.categorical_color(index),
            Self::UserDefined(color) => *color,
        }
    }
}
