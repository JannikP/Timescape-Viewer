use iced::widget::{button, center, column, text};
use iced::{Element, Length};
use rust_i18n::t;

use crate::constants::icons::{CLOSE_ICON, COLLAPSE_VERTICAL_ICON, FULLSCREEN_ICON};
use crate::constants::layout::HANDLE_BAR_WIDTH;
use crate::messages::Message;
use crate::theme::MakoTheme;
use crate::widgets::Hint;

pub fn scope_handle_bar<'b>(index: usize) -> Element<'b, Message, MakoTheme> {
    column![
        center(text(":")) // TODO: Drag area for rearranging scopes.
            .width(HANDLE_BAR_WIDTH)
            .height(Length::Fill),
        button(COLLAPSE_VERTICAL_ICON) // TODO: Minimize scope button
            .width(HANDLE_BAR_WIDTH)
            .height(HANDLE_BAR_WIDTH)
            .hint(t!("timescape.minimize_scope_hint")),
        button(FULLSCREEN_ICON) // TODO: Maximize scope button
            .width(HANDLE_BAR_WIDTH)
            .height(HANDLE_BAR_WIDTH)
            .hint(t!("timescape.maximize_scope_hint")),
        button(CLOSE_ICON)
            .on_press(Message::RemoveScope(index))
            .width(HANDLE_BAR_WIDTH)
            .height(HANDLE_BAR_WIDTH)
            .hint(t!("timescape.remove_scope_hint")),
    ]
    .width(HANDLE_BAR_WIDTH)
    .height(Length::Fill)
    .into()
}
