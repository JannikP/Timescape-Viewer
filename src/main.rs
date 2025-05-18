#![windows_subsystem = "windows"]
mod commands;
mod messages;
mod state;
mod theme;
mod views;

use grid::Grid;
use iced::{Element, Task, Theme};
use iced::widget::{text};

use messages::Message;
use state::{ScopeLegend, ScopePlotter, Window};

pub fn main() -> iced::Result {
    iced::application::application(
        TimescapeViewer::new,
        TimescapeViewer::update,
        TimescapeViewer::view
    )
    .theme(TimescapeViewer::theme)
    .run()
}

#[derive(Default)]
struct TimescapeViewer {
    _scopes: Vec<ScopeLegend>,
    _windows: Vec<Window>,
    _plotters: Grid<ScopePlotter>,
}

impl TimescapeViewer {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::none(),
        )
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, this is iced!").into()
    }
}

#[cfg(test)]
mod tests {
}
