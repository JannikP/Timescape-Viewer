use iced::time::{Duration, Instant};

use crate::core::{Domain, Span};
use crate::state::{ScopeLegend, Window};

/// Interaction that happen less than this duration apart can be merged into one undo-able step.
const MERGE_WINDOW: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub struct Memento {
    /// Point in time when the last interaction was merged into this [Memento].
    /// Used to determine wether the next interaction happens still in the [MERGE_WINDOW].
    latest_interaction: Option<Instant>,

    chart: Option<ChartMemento>,

    window: Option<WindowMemento>,
}

impl Memento {
    pub fn new(now: Instant) -> Self {
        Self {
            latest_interaction: Some(now),
            chart: None,
            window: None,
        }
    }

    pub fn stamp(&mut self, now: Instant) {
        self.latest_interaction = Some(now);
    }

    pub fn zoom_chart(chart: usize, current_domain: Domain, pivot: f32, factor: f32) -> Self {
        Self {
            latest_interaction: None, // Will be stamped when the message is received in `update`.
            chart: Some(ChartMemento {
                affected: chart,
                before: current_domain,
                after: current_domain.zoom(pivot, factor),
            }),
            window: None,
        }
    }

    pub fn can_merge(&self, new: &Self, now: Instant) -> bool {
        // Is the action in the interaction window or is the previous action to long ago?
        let in_interaction_window = if let Some(prev) = self.latest_interaction {
            now - prev < MERGE_WINDOW
        } else {
            // Previous interaction time is not known, just assume it fits.
            true
        };

        // Do the previous action and the new action affect the same chart or none at all?
        let same_charts = match (&self.chart, &new.chart) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => false,
            (
                Some(ChartMemento {
                    affected: previous_affected,
                    before: _,
                    after: _,
                }),
                Some(ChartMemento {
                    affected: now_affected,
                    before: _,
                    after: _,
                }),
            ) => *previous_affected == *now_affected,
        };

        // Do the previous action and the new action affect the same window or none at all?
        let same_windows = match (&self.window, &new.window) {
            (None, None) => true,
            (None, Some(_)) => true,
            (Some(_), None) => false,
            (
                Some(WindowMemento {
                    affected: previous_affected,
                    before: _,
                    after: _,
                }),
                Some(WindowMemento {
                    affected: now_affected,
                    before: _,
                    after: _,
                }),
            ) => *previous_affected == *now_affected,
        };

        // All three conditions must be met
        in_interaction_window && same_charts && same_windows
    }

    pub fn merge(&mut self, new: &Self, now: Instant) {
        // Merge `latest_interaction` => the latest interaction after merging is now.
        self.latest_interaction = Some(now);

        // Merge the chart changes
        match (&self.chart, &new.chart) {
            (None, None) => {} // Nothing to merge in this case
            (None, Some(chart_memento)) => {
                // No chart was changed before. Just copy from message.
                self.chart = Some(chart_memento.clone());
            }
            (Some(_), None) => {
                panic!("Cannot merge a memento that affected a chart with an unspecific one.")
            }
            (
                Some(ChartMemento {
                    affected: previous_affected,
                    before,
                    after: _,
                }),
                Some(ChartMemento {
                    affected: now_affected,
                    before: _,
                    after,
                }),
            ) => {
                assert!(*previous_affected == *now_affected);
                self.chart = Some(ChartMemento {
                    affected: *previous_affected,
                    before: *before,
                    after: *after,
                });
            }
        };

        // Merge the window changes
        match (&self.window, &new.window) {
            (None, None) => {} // Nothing to merge in this case
            (None, Some(window_memento)) => {
                // No chart was changed before. Just copy from message.
                self.window = Some(window_memento.clone());
            }
            (Some(_), None) => {
                panic!("Cannot merge a memento that affected a window with an unspecific one.")
            }
            (
                Some(WindowMemento {
                    affected: previous_affected,
                    before,
                    after: _,
                }),
                Some(WindowMemento {
                    affected: now_affected,
                    before: _,
                    after,
                }),
            ) => {
                assert!(*previous_affected == *now_affected);
                self.window = Some(WindowMemento {
                    affected: *previous_affected,
                    before: *before,
                    after: *after,
                });
            }
        };
    }

    pub fn apply(&self, scopes: &mut Vec<ScopeLegend>, windows: &mut Vec<Window>, now: Instant) {
        if let Some(memento) = &self.chart
            && let Some(scope) = scopes.get_mut(memento.affected)
        {
            match scope {
                ScopeLegend::LineChart(line_chart) => {
                    line_chart.set_domain(memento.after, now);
                }
                ScopeLegend::Spectrogram(_spectrogram) => {}
                ScopeLegend::TrailChart(_trail_chart) => {}
            }
        }

        if let Some(memento) = &self.window
            && let Some(_window) = windows.get_mut(memento.affected)
        {
            // TODO: Set window's span
        }
    }

    pub fn revert(&self, scopes: &mut Vec<ScopeLegend>, windows: &mut Vec<Window>, now: Instant) {
        if let Some(memento) = &self.chart
            && let Some(scope) = scopes.get_mut(memento.affected)
        {
            match scope {
                ScopeLegend::LineChart(line_chart) => {
                    line_chart.set_domain(memento.before, now);
                }
                ScopeLegend::Spectrogram(_spectrogram) => {}
                ScopeLegend::TrailChart(_trail_chart) => {}
            }
        }

        if let Some(memento) = &self.window
            && let Some(_window) = windows.get_mut(memento.affected)
        {
            // TODO: Set window's span
        }
    }
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
