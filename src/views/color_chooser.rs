use iced::Element;
use iced::widget::{column, radio, text};

use crate::constants::layout::PANEL_GAP;
use crate::messages::Message;
use crate::state::color_chooser::{Authority, ColorChooser};
use crate::theme::MakoTheme;

pub fn view_color_chooser<'a, 'b>(chooser: &'a ColorChooser) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    column![
        text("Color chooser dialog"),
        radio(
            "Automatic",
            Authority::Automatic,
            Some(chooser.authority()),
            |_authority| Message::None,
        ),
        radio(
            "User defined",
            Authority::UserDefined,
            Some(chooser.authority()),
            |_authority| Message::None,
        ),
    ]
    .spacing(PANEL_GAP)
    .into()
}
