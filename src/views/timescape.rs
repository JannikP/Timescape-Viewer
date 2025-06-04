use iced::widget::{Space, button, column, row, scrollable, text};
use iced::{Element, Length};
use rust_i18n::t;

use crate::TimescapeViewer;
use crate::constants::icons::MENU_ICON;
use crate::constants::layout::PANEL_GAP;
use crate::messages::Message;
use crate::state::Stage;

pub fn view_timescape(app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        header(app),
        scrollable(content(app)).width(Length::Fill).height(Length::Fill),
        footer(app),
    ]
    .spacing(PANEL_GAP)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn header(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![
        button(MENU_ICON).on_press(Message::GoTo(Stage::Backstage)),
        Space::with_width(Length::Fill),
        Space::with_width(12),
    ]
    .spacing(PANEL_GAP)
    .width(Length::Fill)
    .height(Length::Shrink)
    .into()
}

fn content(_app: &TimescapeViewer) -> Element<'_, Message> {
    text(t!("timescape.greeting")).into()
}

fn footer(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![text("\u{2326} \u{E000} This is the footer."),]
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}
