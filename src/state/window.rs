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
}

impl Window {
    pub fn new(run: Rc<Run>) -> Self {
        Self {
            begin: Timestamp(0),
            end: Timestamp(4000),
            live: LiveMode::Off,
            hover: None,
            first_cursor: None,
            second_cursor: None,
            run,
        }
    }
}
