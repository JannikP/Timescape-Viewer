use super::Scope;

pub struct SpectrogramLegend {}

impl Scope for SpectrogramLegend {
    fn height(&self) -> f32 {
        100.0
    }

    fn create_plotter(&self) -> super::ScopePlotter {
        super::ScopePlotter::Spectrogram(SpectrogramPlotter {})
    }
}

pub struct SpectrogramPlotter {}
