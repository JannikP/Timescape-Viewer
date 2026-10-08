use iced::time::{Duration, Instant};

use crate::core::{Domain, Span};

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
