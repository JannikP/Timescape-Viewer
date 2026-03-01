use crate::constants::layout::{MAXIMUM_SCOPE_HEIGHT, MINIMUM_SCOPE_HEIGHT};

use super::Scope;

#[derive(Debug)]
pub struct SpectrogramLegend {
    index: usize,
    height: f32,
}

impl Default for SpectrogramLegend {
    fn default() -> Self {
        Self {
            index: 0,
            height: 100.0,
        }
    }
}

impl Scope for SpectrogramLegend {
    fn height(&self) -> f32 {
        self.height
    }

    fn resize(&mut self, height: f32) {
        self.height = height.clamp(MINIMUM_SCOPE_HEIGHT, MAXIMUM_SCOPE_HEIGHT);
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
