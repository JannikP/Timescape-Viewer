use super::Scope;

#[derive(Debug, Default, Clone)]
pub struct TrailChartLegend {
    index: usize,
    pub signals: Vec<String>,
}

impl TrailChartLegend {

}

impl Scope for TrailChartLegend {
    fn height(&self) -> f32 {
        32.0
    }

    fn create_plotter(&self) -> super::ScopePlotter {
        super::ScopePlotter::TrailChart(TrailChartPlotter { tracks: Vec::new() })
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index:usize) {
        self.index = index;
    }
}

pub struct TrailChartPlotter {
    tracks: Vec<Trail>,
}

pub struct Trail {}
