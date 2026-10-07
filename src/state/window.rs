use std::rc::Rc;

use crate::core::Span;
use crate::state::{LiveMode, Run, Timestamp};

#[derive(Debug, Clone)]
pub struct Window {
    span: Span,
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
    pub fn new(run: Rc<Run>) -> Self {
        Self {
            span: Span::new(0, 4000),
            live: LiveMode::Off,
            hover: None,
            first_cursor: None,
            second_cursor: None,
            run,
            size: 1,
        }
    }
}
