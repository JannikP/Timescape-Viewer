use std::iter::StepBy;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Row, button, center, column, container, row, scrollable, space, text};
use iced::{Element, Length};
use rust_i18n::t;

use crate::TimescapeViewer;
use crate::constants::icons::{
    LINE_CHART_ICON, MENU_ICON, SKIP_BACK_ICON, SKIP_FORWARD_ICON, SPECTROGRAM_ICON,
    TRAIL_CHART_ICON,
};
use crate::constants::layout::{BUTTON_GAP, LEGEND_WIDTH, PANEL_GAP};
use crate::messages::Message;
use crate::state::{Scope, ScopeLegend, ScopePlotter, Stage, Window};
use crate::theme::{ButtonClass, ContainerClass, MakoTheme};
use crate::views::line_chart::{line_chart_legend, line_chart_plotter};
use crate::views::spectrogram::spectrogram_legend;
use crate::views::trail_chart::trail_chart_legend;
use crate::widgets::{Divider, Hint};

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
pub fn view_timescape(app: &TimescapeViewer) -> Element<'_, Message, MakoTheme> {
    container(
        column![header(app), content(app), footer(app),]
            .spacing(PANEL_GAP)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .class(ContainerClass::Abyss)
    .into()
}

fn header(app: &TimescapeViewer) -> Element<'_, Message, MakoTheme> {
    Row::new()
        .push(toolbar(app))
        .extend(app.windows.iter().map(panorama))
        .push(space().width(12)) // TODO: Put a button to create windows here.
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Shrink)
        .into()
}

fn toolbar(app: &TimescapeViewer) -> Element<'_, Message, MakoTheme> {
    row![
        button(MENU_ICON).on_press(Message::GoTo(Stage::Backstage)),
        button(SKIP_BACK_ICON) // TODO: Replace with proper undo icon.
            .on_press_maybe(app.history.undo_message())
            .class(ButtonClass::Secondary),
        button(SKIP_FORWARD_ICON) // TODO: Replace with proper redo icon.
            .on_press_maybe(app.history.redo_message())
            .class(ButtonClass::Secondary),
    ]
    .spacing(BUTTON_GAP)
    .width(LEGEND_WIDTH)
    .height(Length::Shrink)
    .into()
}

fn panorama(window: &Window) -> Element<'_, Message, MakoTheme> {
    text!("Panorama")
        .width(Length::FillPortion(window.size))
        .height(Length::Fill)
        .into()
}

fn content(app: &TimescapeViewer) -> Element<'_, Message, MakoTheme> {
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
                    button(text(
                        [
                            LINE_CHART_ICON,
                            " ",
                            t!("timescape.no_scope.add_line_chart").as_ref()
                        ]
                        .concat()
                    ))
                    .on_press(Message::AddLineChart),
                    button(text(
                        [
                            SPECTROGRAM_ICON,
                            " ",
                            t!("timescape.no_scope.add_spectrogram").as_ref()
                        ]
                        .concat()
                    ))
                    .on_press(Message::AddSpectrogram),
                    button(text(
                        [
                            TRAIL_CHART_ICON,
                            " ",
                            t!("timescape.no_scope.add_trail_chart").as_ref(),
                        ]
                        .concat()
                    ))
                    .on_press(Message::AddTrailChart),
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
        scrollable(
            Column::new()
                .extend(app.scopes.iter().flat_map(|legend| {
                    [
                        scope(
                            legend,
                            app.windows.as_slice(),
                            app.plotters.iter_row(legend.index()),
                        ),
                        Divider::horizontal(legend.height(), |v| {
                            Message::ResizeScope(legend.index(), v)
                        })
                        .into(),
                    ]
                }))
                .push(space::vertical())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .spacing(2)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

fn scope<'a, 'b>(
    scope: &'a ScopeLegend,
    windows: &'a [Window],
    plotters: StepBy<std::slice::Iter<'a, ScopePlotter>>,
) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    Row::new()
        .push(legend(scope))
        .extend(windows.iter().zip(plotters).flat_map(|window_plotter| {
            [
                plotter(scope, window_plotter),
                Divider::vertical(0.5, |_| Message::None).into(),
            ]
        }))
        .width(Length::Fill)
        .height(scope.height())
        .into()
}

fn legend<'a, 'b>(legend: &'a ScopeLegend) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    let content = match legend {
        ScopeLegend::LineChart(line_chart) => line_chart_legend(line_chart),
        ScopeLegend::Spectrogram(spectrogram) => spectrogram_legend(spectrogram),
        ScopeLegend::TrailChart(trail_chart) => trail_chart_legend(trail_chart),
    };
    container(content)
        .padding(8) // TODO: Make this a constant
        .width(LEGEND_WIDTH)
        .height(Length::Fill)
        .class(ContainerClass::Standard)
        .into()
}

fn plotter<'a, 'b>(
    scope: &'a ScopeLegend,
    (window, plotter): (&'a Window, &'a ScopePlotter),
) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    let content = match (scope, plotter) {
        (ScopeLegend::LineChart(line_legend), ScopePlotter::LineChart(line_plotter)) => {
            line_chart_plotter(line_legend, window, line_plotter)
        }
        (
            ScopeLegend::Spectrogram(_spectrogram_legend),
            ScopePlotter::Spectrogram(_spectrogram_plotter),
        ) => text("TODO: Spectrogram").into(),
        (ScopeLegend::TrailChart(_trail_legend), ScopePlotter::TrailChart(_trail_plotter)) => {
            text("TODO: Trail chart").into()
        }
        _ => panic!("Invalid combination of legend and plotter!"),
    };
    container(content)
        .width(Length::FillPortion(window.size))
        .height(Length::Fill)
        .class(ContainerClass::Standard)
        .into()
}

fn footer(app: &TimescapeViewer) -> Element<'_, Message, MakoTheme> {
    container(
        row![
            button(LINE_CHART_ICON)
                .on_press(Message::AddLineChart)
                .hint(t!("timescape.add_line_chart_hint")),
            button(SPECTROGRAM_ICON)
                .on_press(Message::AddSpectrogram)
                .hint(t!("timescape.add_spectrogram_hint")),
            button(TRAIL_CHART_ICON)
                .on_press(Message::AddTrailChart)
                .hint(t!("timescape.add_trail_chart_hint")),
            text(&app.hint),
        ]
        .align_y(Vertical::Center)
        .spacing(PANEL_GAP)
        .width(Length::Fill)
        .height(Length::Shrink),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .class(ContainerClass::Standard)
    .into()
}
