use std::iter::StepBy;

use iced::alignment::Horizontal;
use iced::widget::{Column, Row, Space, button, center, column, row, text};
use iced::{Element, Length};
use rust_i18n::t;

use crate::TimescapeViewer;
use crate::constants::icons::MENU_ICON;
use crate::constants::layout::PANEL_GAP;
use crate::messages::Message;
use crate::state::{Scope, ScopeLegend, ScopePlotter, Stage, Window};

/// The main view of the app where users actually view the timeseries data using
/// A list of scopes.
///
#[doc = simple_mermaid::mermaid!("timescape_block_diagram.mmd")]
///
/// The frame is divided into a grid of scopes and windows. Vertically there are
/// three main sections:
///
/// 1. The header with a small tool bar and one time line per active
///    [crate::state::Window]. This part is build by the function [header].
/// 2. The main content area is wrapped in a scrollable and further divided
///    into one row per [crate::state::Scope]. This section is build by the
///    function [content].
/// 3. The final row is the footer consisting of buttons to add scopes and a
///    text line displaying useful usage hints. This section is build by the
///    function [footer].
pub fn view_timescape(app: &TimescapeViewer) -> Element<'_, Message> {
    column![header(app), content(app), footer(app),]
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn header(_app: &TimescapeViewer) -> Element<'_, Message> {
    row![
        button(MENU_ICON).on_press(Message::GoTo(Stage::Backstage)),
        Space::with_width(Length::Fill),
        Space::with_width(12), // TODO: Replace this with a button to add more windows
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
                button(text(t!("backstage.open_file"))).on_press(Message::ChooseFile)
            ]
            .spacing(40)
            .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else if app.scopes.is_empty() {
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
            .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        Column::new()
            .extend(app.scopes.iter().enumerate().map(|(index, legend)| {
                scope(legend, app.windows.as_slice(), app.plotters.iter_row(index))
            }))
            .width(Length::Fill)
            .into()
    }
}

fn scope<'a, 'b>(
    legend: &'a ScopeLegend,
    windows: &'a [Window],
    plotters: StepBy<std::slice::Iter<ScopePlotter>>,
) -> Element<'b, Message> {
    Row::new()
        .push(match legend {
            ScopeLegend::LineChart(_) => text("Line chart"),
            ScopeLegend::Spectrogram(_) => text("Spectrogram"),
            ScopeLegend::TrailChart(_) => text("Trail chart"),
        })
        .extend(windows.iter().zip(plotters).map(plotter))
        .height(legend.height())
        .into()
}

fn plotter<'a, 'b>((_window, _plotter): (&'a Window, &'a ScopePlotter)) -> Element<'b, Message> {
    text("Plotter")
        //.width(window.width)
        .into()
}

fn footer(app: &TimescapeViewer) -> Element<'_, Message> {
    row![button("+"), text(&app.hint)]
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}
