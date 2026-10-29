use iced::Element;
use iced::widget::{center, mouse_area, opaque, stack};

use crate::theme::{ContainerClass, MakoTheme};

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
        opaque(mouse_area(center(opaque(content)).class(ContainerClass::Tinted)).on_press(on_blur))
    ]
    .into()
}
