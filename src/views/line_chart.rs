use iced::widget::{
    Column, Row, Space, button, center, column, container, row, space, text, text_input,
};
use iced::{Element, Length};

use crate::constants::icons::SIGNAL_ICON;
use crate::messages::Message;
use crate::state::line_chart::{LineChartLegend, LineChartLegendEntry};

pub fn line_chart_legend<'a, 'b>(legend: &'a LineChartLegend) -> Element<'b, Message>
where
    'a: 'b,
{
    Column::new()
        .extend(legend.iter_signals().map(signal_legend_entry))
        .push(signal_chooser_line())
        .push(space::vertical())
        .spacing(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn signal_legend_entry<'a, 'b>(entry: &'a LineChartLegendEntry) -> Element<'b, Message>
where
    'a: 'b,
{
    row![
        text(SIGNAL_ICON).color(entry.color),
        text(entry.signal.as_str()),
        space::horizontal(),
        button("hide"),
        button("delete"),
    ]
    .spacing(4)
    .into()
}

fn signal_chooser_line<'b>() -> Element<'b, Message> {
    row![text("+"), text_input("Enter signal name here...", ""),].into()
}
