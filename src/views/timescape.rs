use iced::{Element, Font, Length};
use iced::widget::{button, column, row, scrollable, text, Space};

use crate::constants::icons::MENU_ICON;
use crate::constants::layout::PANEL_GAP;
use crate::messages::Message;
use crate::state::Stage;
use crate::TimescapeViewer;

pub fn view_timescape(app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        header(app),
        scrollable(content(app)),
        footer(app),
    ]
    .spacing(PANEL_GAP)
    .height(Length::Fill)
    .into()
}

fn header(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![
        button(MENU_ICON)
            .on_press(Message::GoTo(Stage::Backstage)),
        Space::with_width(Length::Fill),
        Space::with_width(12),
    ].into()
}

fn content(_app: &TimescapeViewer) -> Element<'_, Message> {
    text("Hello, this is the timescape!").into()
}

fn footer(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![
        text("\u{2326} Hello, this is the timescape!"),
        text("\u{E000} Hello, this is the timescape!")
            .font(Font::with_name("FiraSansCondensed-Regular-Expanded")),
        text("\u{E000} Hello, this is the timescape!")
            .font(Font::with_name("Fira Sans Condensed")),
        text("\u{0E000} Hello, this is the timescape!")
            .font(Font::with_name("Fira Sans Condensed Regular")),
        text("\u{E000} Hello, this is the timescape!")
            .font(Font::with_name("Overpass Mono")),
    ]
    .spacing(PANEL_GAP)
    .width(Length::Fill)
    .into()
}
