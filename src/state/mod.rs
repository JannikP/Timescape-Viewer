//! Contains the state and data tree of the entire application.
#![doc = simple_mermaid::mermaid!("overview.mmd")]

pub mod line_chart;
pub mod signal;
pub mod spectrogram;
pub mod trail_chart;
pub mod window;

use std::{f64, rc::Rc};

use crate::origins::csv::CsvOptions;

use enum_dispatch::enum_dispatch;
use line_chart::{LineChartLegend, LineChartPlotter};
use signal::Signal;
use spectrogram::{SpectrogramLegend, SpectrogramPlotter};
use trail_chart::{TrailChartLegend, TrailChartPlotter};

pub use window::Window;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Stage {
    Backstage,
    #[default]
    Timescape,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Modal {
    #[default]
    None,
    InterpretCsv(CsvOptions),
}

#[enum_dispatch(ScopeLegend)]
pub trait Scope {
    fn height(&self) -> f32;

    fn create_plotter(&self) -> ScopePlotter;
}

#[enum_dispatch]
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

#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Timestamp(f64);

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum LiveMode {
    #[default]
    Off,
    Grow,
    Move,
}

pub struct Source {
    runs: Vec<Rc<Run>>,
}

impl Source {
    pub fn new_example_sines() -> Self {
        Self {
            runs: vec![
                Rc::new(Run::new_example_sines()),
            ]
        }
    }
}

#[derive(Debug)]
pub struct Run {
    title: String,
    tracks: Vec<Track>,
    // TODO: Add meta data
}

impl Run {
    pub fn new_example_sines() -> Self {
        Run {
            title: "Example sines".to_string(),
            tracks: vec![
                Track {
                    signals: vec![
                        Signal::new("440 Hz sine")
                            .with_description("A sine wave at 440 Hz")
                            .without_unit()
                            .reference_counted(),
                        Signal::new("Mains voltage")
                            .with_description("The european main voltage (50 Hz, 230 V AC)")
                            .with_unit("V")
                            .reference_counted(),
                    ],
                    time: Timeline::fixed_sample_rate(10000.0, 100_000),
                    values: vec![
                        demo_sine(10000.0, 100_000, 440.0, 1.0),
                        demo_sine(10000.0, 100_000,  50.0, 325.2691193),
                    ],
                }
            ],
        }
    }
}

fn demo_sine(sample_rate: f64, sample_count: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    let a = f64::consts::TAU * frequency / sample_rate;
    let mut values = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let value = (i as f64 * a).sin() * amplitude;
        values.push(value);
    }
    values
}

#[derive(Debug)]
pub struct Track {
    signals: Vec<Rc<Signal>>,
    time: Timeline,
    values: Vec<Vec<f64>>,
    // TODO: Add optional absolute time offset (the time of the first sample)
}

#[derive(Debug)]
pub enum Timeline {
    FixedSampleRate {
        delta_time: f64,
        sample_count: usize,
    },

    #[allow(dead_code)]
    ExplicitTime {
        timestamps: Vec<f64>,
    }

    // TODO: VariableTimeSteps { time_steps: Vec<f64> },
}

impl Timeline {
    pub fn fixed_sample_rate(sample_rate: f64, sample_count: usize) -> Self {
        Self::FixedSampleRate {
            delta_time: 1.0 / sample_rate,
            sample_count,
        }
    }
}


