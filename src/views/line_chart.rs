use iced::alignment::Vertical;
use iced::widget::{Column, button, row, space, text, text_input};
use iced::{Element, Length};
use rust_i18n::t;

use crate::constants::icons::{ADD_ICON, DELETE_ICON, HIDE_ICON, SHOW_ICON, SIGNAL_ICON};
use crate::messages::Message;
use crate::state::Scope;
use crate::state::line_chart::{LineChartLegend, LineChartLegendEntry};
use crate::views::common::scope_handle_bar;
use crate::widgets::Hint;

pub fn line_chart_legend<'a, 'b>(legend: &'a LineChartLegend) -> Element<'b, Message>
where
    'a: 'b,
{
    row![
        scope_handle_bar(legend.index()),
        Column::new()
            .extend(
                legend
                    .iter_signals()
                    .map(|e| signal_legend_entry(legend, e))
            )
            .push(signal_chooser_line(legend))
            .push(space::vertical())
            .spacing(4)
            .width(Length::Fill)
            .height(Length::Fill),
        // TODO: Axis
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn signal_legend_entry<'a, 'b>(
    scope: &'a LineChartLegend,
    entry: &'a LineChartLegendEntry,
) -> Element<'b, Message>
where
    'a: 'b,
{
    row![
        text(SIGNAL_ICON).color(entry.color),
        text(entry.signal.as_str()),
        space::horizontal(),
        button(if entry.visible { SHOW_ICON } else { HIDE_ICON })
            .on_press(entry.toggle_visibility_message(scope))
            .hint(t!("line_chart.visibility_hint")),
        button(DELETE_ICON)
            .on_press(entry.delete_message(scope))
            .hint(t!("line_chart.delete_hint")),
    ]
    .align_y(Vertical::Center)
    .spacing(4)
    .into()
}

fn signal_chooser_line<'a, 'b>(legend: &'a LineChartLegend) -> Element<'b, Message>
where
    'a: 'b,
{
    row![
        text(ADD_ICON),
        text_input(
            &t!("line_chart.add_signal.placeholder"),
            legend.signal_input()
        )
        .on_input(legend.on_signal_input())
        .on_submit(legend.on_signal_input_submit())
        .hint(t!("line_chart.add_signal.hint")),
    ]
    .align_y(Vertical::Center)
    .spacing(4)
    .into()
}
