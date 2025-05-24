// Hide console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod constants;
mod logging;
mod messages;
mod state;
mod theme;
mod views;

use grid::Grid;
use iced::{Element, Font, Settings, Task, Theme};
use iced::font::{Family, Weight, Stretch, Style};
use iced::widget::center;
use log::info;

use logging::setup_logger;
use messages::Message;
use state::{ScopeLegend, ScopePlotter, Stage, Window};
use views::{view_backstage, view_timescape};

pub fn main() -> iced::Result {
    setup_logger();
    iced::application::application(
        TimescapeViewer::new,
        TimescapeViewer::update,
        TimescapeViewer::view
    )
    .settings(Settings {
        id: Some("org.timescape-viewer.application".into()),
        fonts: vec![
            include_bytes!("../assets/fonts/FiraSansCondensed-Regular-Expanded.ttf").into(),
        ],
        default_font: Font {
            family: Family::Name("Fira Sans Condensed"),
            weight: Weight::Normal,
            stretch: Stretch::Condensed,
            style: Style::Normal,
        },
        default_text_size: 16.into(),
        antialiasing: true,
    })
    .theme(TimescapeViewer::theme)
    .run()
}

#[derive(Default)]
struct TimescapeViewer {
    stage: Stage,
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

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoTo(stage) => {
                info!("Going to {:?}", stage);
                self.stage = stage;
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match self.stage {
            Stage::Backstage => view_backstage(self),
            Stage::Timescape => view_timescape(self),
        };
        center(content).into()
    }
}

#[cfg(test)]
mod tests {
}
