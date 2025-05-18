//! Contains the state and data tree of the entire application.
#![doc = simple_mermaid::mermaid!("overview.mmd")]

pub mod line_chart;
pub mod signal;
pub mod spectrogram;
pub mod trail_chart;

use std::rc::Rc;

use line_chart::{LineChartLegend, LineChartPlotter};
use signal::Signal;
use spectrogram::{SpectrogramLegend, SpectrogramPlotter};
use trail_chart::{TrailChartLegend, TrailChartPlotter};

pub const BLOCK_SIZE: usize = 65536;

pub enum ScopeLegend {
    LineChart(LineChartLegend),
    Spectrogram(SpectrogramLegend),
    TrailChart(TrailChartLegend),
}

pub enum ScopePlotter {
    LineChart(LineChartPlotter),
    Spectrogram(SpectrogramPlotter),
    TrailChart(TrailChartPlotter),
}

pub struct Window {
    begin: Timestamp,
    end: Timestamp,
    live: LiveMode,
    hover: Option<Timestamp>,
    first_cursor: Option<Timestamp>,
    second_cursor: Option<Timestamp>,
    run: Rc<Run>,
}

#[repr(transparent)]
pub struct Timestamp(f64);

pub enum LiveMode {
    Off,
    Grow,
    Move,
}

pub struct Source {
    runs: Vec<Rc<Run>>,
}

pub struct Run {
    title: String,
    tracks: Vec<Track>,
}

pub struct Track {
    signals: Vec<Rc<Signal>>,
    blocks: Vec<Block>,
}

pub struct Block {
    timestamps: [f64; BLOCK_SIZE],
    entries: Box<[f64]>,
    begin: Timestamp,
    end: Timestamp,
}
