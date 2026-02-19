use std::rc::Rc;

use super::{LiveMode, Run, Timestamp};

#[derive(Debug, Clone)]
pub struct Window {
    begin: Timestamp,
    end: Timestamp,
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
            begin: Timestamp(0.0),
            end: Timestamp(0.0),
            live: LiveMode::Off,
            hover: None,
            first_cursor: None,
            second_cursor: None,
            run,
            size: 1,
        }
    }
}
