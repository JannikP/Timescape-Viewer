use iced::Color;

use super::Scope;

pub struct LineChartLegend {
    signals: Vec<LineChartLegendEntry>,
    minimum: f64,
    maximum: f64,
    height: f32,
}

impl Scope for LineChartLegend {
    fn height(&self) -> f32 {
        self.height
    }
}

pub struct LineChartLegendEntry {
    signal: String,
    color: Color,
}

pub struct LineChartPlotter {
    signals: Vec<Line>,
}

pub struct Line {}
