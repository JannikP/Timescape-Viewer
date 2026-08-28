use std::sync::Arc;

use crate::core::{Bracket, Span};
use crate::state::signal::{Signal, SignalId};

pub trait Repository {
    // General requirements:
    //  - Read while write across multiple threads.
    //  - Multiple concurrent readers. E. g. export task and user interface rendering.
    //  - virtually unbounded measurements. Unload cold data to SQLite database, keep hot data in
    //    memory.
    //  -

    // ============================== Meta Data ===============================

    fn name(&self) -> &str;

    // ========================== Signal Management ===========================

    fn add_signal(&self, signal: Signal) -> SignalId;

    fn find_signal(&self, name: &str) -> Option<Arc<Signal>>;

    fn get_signal(&self, id: SignalId) -> Option<Arc<Signal>>;

    fn signals(&self) -> impl Iterator<Item = Arc<Signal>>;

    // ============================= Adding Data ==============================

    /// Add a single sample with one timestamp and one value for a single signal.
    /// Prefer [Repository::add_row] or [Repository::add_column] for better
    /// performance where possible.
    fn add_sample(&self, timestamp: i64, signal: SignalId, value: f64);

    /// Add a sample with one timestamp but values for several signals. E. g.
    /// one row of a .csv file.
    fn add_row(&self, timestamp: i64, signals: &[SignalId], values: &[f64]);

    /// Add many samples with many timestamps but only for a single signal.
    /// Meaning one single value per timestamp. E. g. importing a one block of
    /// an .mdf file.
    fn add_column(&self, signal: SignalId, timestamps: &[i64], values: &[f64]);

    // ================================ Status ================================

    fn total_history(&self) -> Span;

    fn is_still_recording(&self) -> bool;

    // =============================== Querying ===============================

    // Use cases:
    //  - Query slices within a window for line chart
    //  - Query spans within a window for a trail chart
    //  - Query overlapping sample lines within a window for a spectrogram
    //  - Query resampled data for export
    //  - Query raw samples for export
    //  - Query aggregated values for statics

    // Variants:
    //  - Absolute date and time vs. relative timestamp
    //  - Efficient multi-threading internally vs. externally vs. none vs. async.
    //  - Values as native u8/i8/u16/i16/.../f32/f64 vs. fixed f64
    //  - What about non continuous values (bit lanes, events, enums, strings)

    fn select_single(&self, signal: SignalId) -> impl QueryBuilder;

    fn select_multiple(&self, signals: &[SignalId]) -> impl QueryBuilder;
}

pub trait QueryBuilder {
    /// Query data starting from this timestamp (inclusive).
    /// Use together with [QueryBuilder::end_at] to select a window.
    fn start_from(self, time: i64) -> Self;

    /// Query data up to and including this timestamp.
    /// Use together with [QueryBuilder::start_from] to select a window.
    fn end_at(self, time: i64) -> Self;

    /// Aggregate samples in ([Bracket])s with a duration of `bracket_length` per bracket.
    /// If there is more than one sample per bracket, compute some statistics per bracket,
    /// such as the average, minium and maximum values.
    fn aggregated(self, bracket_length: std::time::Duration) -> impl Iterator<Item = Bracket>;

    /// Query the raw samples without aggregation. Useful for lossless exports.
    fn samples(self) -> impl Iterator<Item = Sample>;

    /// Resample the data, apply a window function and retrieve every window as `&[f64]` slice.
    /// Useful for FFT analysis of a signal.
    fn windowed(self, weights: &[f64]) -> impl Iterator<Item = &[f64]>;

    /// Resample the signal to a fixed sample rate given by the sample `interval`. Retrieves
    /// the raw samples after resampling without aggregation. Useful for exporters with
    /// different sampling requirements or capabilities compared to the source.
    fn resampled(self, interval: std::time::Duration) -> impl Iterator<Item = Sample>;

    ///
    fn spans(self, min_run_length: i64, hysteresis: impl Fn(f64, bool) -> bool) -> impl Iterator<Item = Run>;
}

pub struct Sample {
    timestamp: i64,
    value: f64,
}

pub struct Run {
    span: Span,
    value: RunValue,
}

pub enum RunValue {
    Low,
    High,
    Mixed(f64),
}
