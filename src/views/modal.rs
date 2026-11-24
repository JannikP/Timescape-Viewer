use iced::Element;
use iced::widget::{center, container, mouse_area, opaque, stack};

use crate::constants::layout::MODAL_PADDING;
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
        opaque(
            mouse_area(
                center(opaque(
                    container(content)
                        .class(ContainerClass::Elevated)
                        .padding(MODAL_PADDING)
                ))
                .class(ContainerClass::Tinted)
            )
            .on_press(on_blur)
        )
    ]
    .into()
}
