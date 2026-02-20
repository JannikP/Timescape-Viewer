use super::Scope;

#[derive(Debug, Default)]
pub struct SpectrogramLegend {
    index: usize,
}

impl Scope for SpectrogramLegend {
    fn height(&self) -> f32 {
        100.0
    }

    fn create_plotter(&self) -> super::ScopePlotter {
        super::ScopePlotter::Spectrogram(SpectrogramPlotter {})
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

pub struct SpectrogramPlotter {}
