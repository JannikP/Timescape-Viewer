use iced::widget::{Button, MouseArea, TextInput, mouse_area};

use crate::messages::Message;

pub trait Hint<'a, Theme> {
    fn hint(self, text: impl Into<String>) -> MouseArea<'a, Message, Theme>;
}

impl<'a, Theme> Hint<'a, Theme> for Button<'a, Message, Theme>
where
    Theme: iced::widget::button::Catalog + 'a,
{
    fn hint(self, hint: impl Into<String>) -> MouseArea<'a, Message, Theme> {
        mouse_area(self)
            .on_enter(Message::Hint(hint.into()))
            .on_exit(Message::Hint(String::new()))
    }
}

impl<'a, Theme> Hint<'a, Theme> for TextInput<'a, Message, Theme>
where
    Theme: iced::widget::text_input::Catalog + 'a,
{
    fn hint(self, hint: impl Into<String>) -> MouseArea<'a, Message, Theme> {
        mouse_area(self)
            .on_enter(Message::Hint(hint.into()))
            .on_exit(Message::Hint(String::new()))
    }
}
