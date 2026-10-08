use crate::core::{Domain, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub chart: Option<ChartMessage>,
    pub windows: Option<ChartMessage>,
}

impl Message {
    pub fn zoom_chart(chart: usize, domain: Domain, pivot: f32, factor: f32) -> Self {
        Self {
            chart: Some(ChartMessage {
                affected: chart,
                target: domain.zoom(pivot, factor),
            }),
            windows: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartMessage {
    pub affected: usize,
    pub target: Domain,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowsMessage {
    pub affected: usize,
    pub target: Span,
}
