use iced::{Element, Length};
use iced::widget::{button, center, column, row, text, Space};

use crate::messages::Message;
use crate::state::Stage;
use crate::TimescapeViewer;

pub fn view_backstage(app: &TimescapeViewer) -> Element<'_, Message> {
    row![
        navigation(app),
        center(text("Hello, this is the backstage!")),
    ].into()
}

fn navigation(_app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        button(">")
            .on_press(Message::GoTo(Stage::Timescape)),
        Space::with_height(Length::FillPortion(1)),
        button("Open File")
            .width(Length::Fill),
        button("Open...")
            .width(Length::Fill),
        Space::with_height(Length::FillPortion(1)),
        button("Information")
            .width(Length::Fill),
        button("Runs")
            .width(Length::Fill),
        button("Export")
            .width(Length::Fill),
        Space::with_height(Length::FillPortion(3)),
        button("Preferences")
            .width(Length::Fill),
    ]
    .spacing(8)
    .width(150)
    .into()
}
