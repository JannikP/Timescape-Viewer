use super::Scope;

pub struct SpectrogramLegend {}

impl Scope for SpectrogramLegend {
    fn height(&self) -> f32 {
        100.0
    }
}

pub struct SpectrogramPlotter {}
