use iced::alignment::Horizontal;
use iced::widget::{button, center, column, row, text, Column, Row, Space};
use iced::{Element, Length};
use rust_i18n::t;

use crate::TimescapeViewer;
use crate::constants::icons::MENU_ICON;
use crate::constants::layout::PANEL_GAP;
use crate::messages::Message;
use crate::state::{Scope, ScopeLegend, Stage};

pub fn view_timescape(app: &TimescapeViewer) -> Element<'_, Message> {
    column![
        header(app),
        content(app),
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

fn content(app: &TimescapeViewer) -> Element<'_, Message> {
    if app.windows.is_empty() {
        center(
            column![
                text(t!("timescape.no_origin.caption")).size(32),
                text(t!("timescape.no_origin.message")),
                button("Open file")
            ]
            .spacing(40)
            .align_x(Horizontal::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    else if app.scopes.is_empty() {
        center(
            column![
                text(t!("timescape.no_scope.caption")).size(32),
                text(t!("timescape.no_scope.message")),
                row![
                    button("Add line chart"),
                    button("Add spectrogram"),
                    button("Add trail chart"),
                ]
                .spacing(20)
            ]
            .spacing(40)
            .align_x(Horizontal::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
    else {
        Column::new()
        .extend(
            app
            .scopes
            .iter()
            .enumerate()
            .map(|(_index, legend)| scope(legend))
        )
        .width(Length::Fill)
        .into()
    }
}

fn scope(legend: &ScopeLegend) -> Element<'_, Message> {
    Row::new()
    .push(
        text("Some scope")
    )
    .height(legend.height())
    .into()
}

fn footer(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![text("\u{2326} \u{E000} This is the footer."),]
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}
