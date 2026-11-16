use iced::time::Instant;
use std::rc::Rc;

use crate::animation::Spring;
use crate::core::{CursorId, Span};
use crate::state::{LiveMode, Run, Timestamp};

#[derive(Debug, Clone)]
pub struct Window {
    index: usize,
    span: Spring<Span>,
    live: LiveMode,
    hover: Option<Timestamp>,
    first_cursor: Option<Timestamp>,
    second_cursor: Option<Timestamp>,
    run: Rc<Run>,

    /// The window fills a portion of the remaining space relative to other
    /// windows.
    /// See [`iced::Length`].
    pub size: u16,
}

impl Window {
    pub fn new(run: Rc<Run>, index: usize) -> Self {
        Self {
            index,
            span: Spring::smooth(Span::new(0, 4000)),
            live: LiveMode::Off,
            hover: None,
            first_cursor: None,
            second_cursor: None,
            run,
            size: 1,
        }
    }

    /// The position of this window in the list of windows.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Sets the position of this window in the list of windows. Used when
    /// windows are pushed or closed to keep the indices in sync.
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// The timestamp of the primary cursor, if shown.
    pub fn primary_cursor(&self) -> Option<i64> {
        self.first_cursor.map(Timestamp::value)
    }

    /// The timestamp of the secondary cursor, if shown.
    pub fn secondary_cursor(&self) -> Option<i64> {
        self.second_cursor.map(Timestamp::value)
    }

    /// Shows the given cursor at the timestamp.
    pub fn set_cursor(&mut self, cursor: CursorId, position: i64) {
        let timestamp = Timestamp::from(position);
        match cursor {
            CursorId::Primary => self.first_cursor = Some(timestamp),
            CursorId::Secondary => self.second_cursor = Some(timestamp),
        }
    }

    /// Hides the given cursor.
    pub fn dismiss_cursor(&mut self, cursor: CursorId) {
        match cursor {
            CursorId::Primary => self.first_cursor = None,
            CursorId::Secondary => self.second_cursor = None,
        }
    }

    pub fn is_animating(&self) -> bool {
        self.span.is_animating()
    }

    pub fn update(&mut self, now: Instant) {
        self.span.update(now);
    }

    pub fn span(&self) -> &Span {
        self.span.value()
    }

    pub fn set_span(&mut self, span: Span, now: Instant) {
        self.span.go(span, now);
    }
}
