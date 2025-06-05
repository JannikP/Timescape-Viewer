use super::Scope;

pub struct TrailChartLegend {
    signals: Vec<String>,
}

impl Scope for TrailChartLegend {
    fn height(&self) -> f32 {
        32.0
    }
}

pub struct TrailChartPlotter {
    tracks: Vec<Trail>,
}

pub struct Trail {}
