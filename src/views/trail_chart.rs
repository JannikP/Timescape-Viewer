use iced::widget::{row, text};
use iced::{Element, Length};

use crate::messages::Message;
use crate::state::Scope;
use crate::state::trail_chart::TrailChartLegend;
use crate::theme::MakoTheme;
use crate::views::common::scope_handle_bar;

pub fn trail_chart_legend<'a, 'b>(legend: &'a TrailChartLegend) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    row![
        scope_handle_bar(legend.index()),
        text("Trail chart"),
        // TODO: Axis
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
