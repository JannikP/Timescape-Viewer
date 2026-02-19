use iced::{Color, Task};

use crate::messages::{Message, line_chart::LineChartMessage};

use super::Scope;

#[derive(Debug, Clone)]
pub struct LineChartLegend {
    index: usize,
    signals: Vec<LineChartLegendEntry>,
    minimum: f64,
    maximum: f64,
    height: f32,
}

impl LineChartLegend {
    pub fn push_signal<S: Into<String>>(&mut self, signal: S) {
        let index = self.signals.len();
        let entry = LineChartLegendEntry {
            index,
            signal: signal.into(),
            color: Color::WHITE,
            visible: true,
        };
        self.signals.push(entry);
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
        }
        Task::none()
    }
}

impl Scope for LineChartLegend {
    fn height(&self) -> f32 {
        self.height
    }

    fn create_plotter(&self) -> super::ScopePlotter {
        super::ScopePlotter::LineChart(LineChartPlotter {
            signals: Vec::new(),
        })
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index:usize) {
        self.index = index;
    }
}

impl Default for LineChartLegend {
    fn default() -> Self {
        Self {
            index: 0,
            signals: Vec::new(),
            minimum: 0.0,
            maximum: 1.0,
            height: 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineChartLegendEntry {
    pub signal: String,
    pub color: Color,
    pub visible: bool,
    pub index: usize,
}

impl LineChartLegendEntry {
    pub fn toggle_visibility_message(&self, scope: &LineChartLegend) -> Message {
        Message::LineChartMessage(scope.index(), LineChartMessage::ToggleVisibility {
            signal: self.index,
            visible: !self.visible,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LineChartPlotter {
    signals: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {}
