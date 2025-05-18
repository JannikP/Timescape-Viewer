use iced::Color;

pub struct LineChartLegend {
    signals: Vec<LineChartLegendEntry>,
    minimum: f64,
    maximum: f64,
}

pub struct LineChartLegendEntry {
    signal: String,
    color: Color,
}

pub struct LineChartPlotter {
    signals: Vec<Line>,
}

pub struct Line {

}
