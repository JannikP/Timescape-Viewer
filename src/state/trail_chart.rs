use super::Scope;

#[derive(Debug, Default, Clone)]
pub struct TrailChartLegend {
    pub signals: Vec<String>,
}

impl Scope for TrailChartLegend {
    fn height(&self) -> f32 {
        32.0
    }

    fn create_plotter(&self) -> super::ScopePlotter {
        super::ScopePlotter::TrailChart(TrailChartPlotter { tracks: Vec::new() })
    }
}

pub struct TrailChartPlotter {
    tracks: Vec<Trail>,
}

pub struct Trail {}
