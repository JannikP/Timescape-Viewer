use iced::Element;
use iced::widget::{button, column, text};

use crate::messages::Message;
use crate::state::Stage;
use crate::TimescapeViewer;

pub fn view_backstage(_app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        button(">").on_press(Message::GoTo(Stage::Timescape)),
        text("Hello, this is the backstage!"),
    ].into()
}

pub fn view_timescape(_app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        button("=").on_press(Message::GoTo(Stage::Backstage)),
        text("Hello, this is the timescape!"),
    ].into()
}
