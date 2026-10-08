use iced::time::{Duration, Instant};

use crate::{
    core::{Domain, Span},
    messages::navigation,
    state::{ScopeLegend, Window},
};

/// Interaction that happen less than this duration apart can be merged into one undo-able step.
const MERGE_WINDOW: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub struct Memento {
    /// Point in time when the last interaction was merged into this [Memento].
    /// Used to determine wether the next interaction happens still in the [MERGE_WINDOW].
    latest_interaction: Instant,

    chart: Option<ChartMemento>,

    window: Option<WindowMemento>,
}

impl Memento {
    pub fn new(now: Instant) -> Self {
        Self {
            latest_interaction: now,
            chart: None,
            window: None,
        }
    }

    pub fn can_merge(&self, _action: &navigation::Message, now: &Instant) -> bool {
        *now - self.latest_interaction < MERGE_WINDOW
    }

    pub fn merge(&mut self, action: &navigation::Message, now: &Instant) {
        self.latest_interaction = *now;
        // TODO: Check if same objects are affected and merge
    }

    pub fn apply(&self, scopes: &mut Vec<ScopeLegend>, windows: &mut Vec<Window>) {
        if let Some(memento) = &self.chart
            && let Some(scope) = scopes.get_mut(memento.affected)
        {
            match scope {
                ScopeLegend::LineChart(line_chart) => {
                    // TODO: Set domain
                }
                ScopeLegend::Spectrogram(_spectrogram) => {}
                ScopeLegend::TrailChart(_trail_chart) => {}
            }
        }

        if let Some(memento) = &self.window
            && let Some(window) = windows.get_mut(memento.affected)
        {
            // TODO: Set window's span
        }
    }

    pub fn revert(&self, scopes: &mut Vec<ScopeLegend>, windows: &mut Vec<Window>) {}
}

#[derive(Debug, Clone, PartialEq)]
struct ChartMemento {
    affected: usize,
    before: Domain,
    after: Domain,
}

#[derive(Debug, Clone, PartialEq)]
struct WindowMemento {
    affected: usize,
    before: Span,
    after: Span,
}
