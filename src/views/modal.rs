use iced::Element;
use iced::widget::{center, mouse_area, opaque, stack};

use crate::theme::MakoTheme;

pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message, MakoTheme>>,
    content: impl Into<Element<'a, Message, MakoTheme>>,
    on_blur: Message,
) -> Element<'a, Message, MakoTheme>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(mouse_area(center(opaque(content))).on_press(on_blur))
    ]
    .into()
}
