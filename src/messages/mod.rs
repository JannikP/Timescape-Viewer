pub mod line_chart;
pub mod window;

use crate::messages::line_chart::LineChartMessage;
use crate::origins::Origin;
use crate::state::{Memento, Stage};

#[derive(Debug, Clone)]
pub enum Message {
    None,
    AbortModal,
    ChooseFile,
    GoTo(Stage),
    Hint(String),
    Open(Origin),
    AddLineChart,
    AddSpectrogram,
    AddTrailChart,
    RemoveScope(usize),
    ResizeScope(usize, f32),
    LineChartMessage(usize, LineChartMessage),
    Window(usize, window::Message),
    Undo,
    Redo,
    Navigation(Box<Memento>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_size_small() {
        assert!(
            std::mem::size_of::<Message>() <= 64,
            "The message should be less than or equal to 64 bytes in size. Box larger content!\nThe actual size is {} bytes.",
            std::mem::size_of::<Message>(),
        );
    }
}
