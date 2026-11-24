use iced::{Element, Padding};
use iced::widget::{Canvas, button, canvas, column, container, radio, row, space, text};

use crate::constants::layout::{BUTTON_GAP, PANEL_GAP};
use crate::messages::Message;
use crate::messages::color_chooser::ColorChooserMessage;
use crate::state::color_chooser::{Authority, ColorChooser};
use crate::theme::{ButtonClass, ContainerClass, MakoTheme};
use crate::widgets::WheelProgram;

pub fn view_color_chooser<'a, 'b>(chooser: &'a ColorChooser) -> Element<'b, Message, MakoTheme>
where
    'a: 'b,
{
    column![
        text!("Color chooser dialog"),
        radio(
            "Automatic",
            Authority::Automatic,
            Some(chooser.authority()),
            |authority| Message::ColorChooserMessage(ColorChooserMessage::ChangeAuthority(authority)),
        ),
        container(text!("Let the theme decide the color."))
            .padding(Padding::new(0.0).left(16.0))
            .class(ContainerClass::Elevated),
        radio(
            "User defined",
            Authority::UserDefined,
            Some(chooser.authority()),
            |authority| Message::ColorChooserMessage(ColorChooserMessage::ChangeAuthority(authority)),
        ),
        container(color_wheel(chooser))
            .padding(Padding::new(0.0).left(16.0))
            .class(ContainerClass::Elevated),
        row![
            space(),
            button("Apply")
                .on_press(chooser.message())
                .class(ButtonClass::Standard),
            button("Cancel")
                .on_press(Message::AbortModal)
                .class(ButtonClass::Secondary),
        ]
            .spacing(BUTTON_GAP)
    ]
    .spacing(PANEL_GAP)
    .into()
}

fn color_wheel<'a, 'b>(chooser: &'a ColorChooser) -> Canvas<WheelProgram<Message>, Message, MakoTheme>
where
    'a: 'b,
{
    canvas(
        WheelProgram::new(
            chooser.hue(),
            chooser.saturation(),
            chooser.value(),
            |hue, saturation| Message::ColorChooserMessage(ColorChooserMessage::ChangeColor { hue, saturation }),
        )
    )
        .width(200)
        .height(200)
}
