use iced::widget::{row, text};
use iced::{Element, Length};

use crate::messages::Message;
use crate::state::Scope;
use crate::state::spectrogram::SpectrogramLegend;
use crate::views::common::scope_handle_bar;

pub fn spectrogram_legend<'a, 'b>(legend: &'a SpectrogramLegend) -> Element<'b, Message>
where
    'a: 'b,
{
    row![
        scope_handle_bar(legend.index()),
        text("Spectrogram"),
        // TODO: Axis
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
