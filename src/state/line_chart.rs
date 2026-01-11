use iced::Color;

use super::Scope;

#[derive(Debug, Clone)]
pub struct LineChartLegend {
    signals: Vec<LineChartLegendEntry>,
    minimum: f64,
    maximum: f64,
    height: f32,
}

impl LineChartLegend {
    pub fn push_signal<S: Into<String>>(&mut self, signal: S) {
        let entry = LineChartLegendEntry {
            signal: signal.into(),
            color: Color::WHITE,
        };
        self.signals.push(entry);
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
}

impl Default for LineChartLegend {
    fn default() -> Self {
        Self {
            signals: Vec::new(),
            minimum: 0.0,
            maximum: 1.0,
            height: 32.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineChartLegendEntry {
    signal: String,
    color: Color,
}

#[derive(Debug, Clone)]
pub struct LineChartPlotter {
    signals: Vec<Line>,
}

#[derive(Debug, Clone)]
pub struct Line {}
