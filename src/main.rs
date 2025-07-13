// Hide console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod constants;
mod logging;
mod messages;
mod origins;
mod state;
mod theme;
mod views;
mod widgets;

use std::rc::Rc;

use grid::Grid;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::{center, text};
use iced::{Element, Font, Settings, Task, Theme};
use log::{debug, info};
use rust_i18n::{i18n, t};

use commands::choose_file::choose_file;
use logging::setup_logger;
use messages::Message;
use state::{Modal, ScopeLegend, ScopePlotter, Stage, Window};
use views::{modal, view_backstage, view_timescape};

use crate::origins::Origin;
use crate::state::{Run, Scope, Source};

// Load translations with configuration from `[package.metadata.i18n]` section in `Cargo.toml`.
i18n!("assets/i18n");

pub fn main() -> iced::Result {
    setup_logger();
    iced::application::application(
        TimescapeViewer::new,
        TimescapeViewer::update,
        TimescapeViewer::view,
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
    .title(TimescapeViewer::title)
    .run()
}

#[derive(Default)]
struct TimescapeViewer {
    stage: Stage,
    modal: Modal,
    scopes: Vec<ScopeLegend>,
    windows: Vec<Window>,
    plotters: Grid<ScopePlotter>,
    sources: Vec<Source>,
    hint: String,
}

impl TimescapeViewer {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AbortModal => {
                self.modal = Modal::None;
                Task::none()
            }
            Message::ChooseFile => {
                Task::perform(choose_file(), |maybe_origin| match maybe_origin {
                    Some(origin) => Message::Open(origin),
                    None => Message::None,
                })
            }
            Message::GoTo(stage) => {
                info!("Going to {:?}", stage);
                self.stage = stage;
                Task::none()
            }
            Message::Hint(hint) => {
                self.hint = hint;
                Task::none()
            }
            Message::Open(ref _origin @ Origin::CsvFile(ref file, ref options)) => {
                info!("Opening {:?} with options {:?}", file, options);
                self.stage = Stage::Timescape;
                self.modal = Modal::InterpretCsv(**options);
                Task::none()
            }
            Message::None => {
                debug!("Do nothing.");
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = match self.stage {
            Stage::Backstage => view_backstage(self),
            Stage::Timescape => view_timescape(self),
        };
        if self.modal != Modal::None {
            center(modal(
                content,
                text("Some modal dialog"),
                Message::AbortModal,
            ))
            .into()
        } else {
            center(content).into()
        }
    }

    fn title(&self) -> String {
        t!("title").to_string()
    }

    /// Adds a new window for the given [Run]. The window will be last one in
    /// the list, meaning it will be displayed on the right. The user can
    /// reorder them afterwards if desired.
    fn push_window(&mut self, run: Rc<Run>) {
        let window = Window::new(run);
        let plotters = self
            .scopes
            .iter()
            .map(|scope| scope.create_plotter())
            .collect();
        self.plotters.push_col(plotters);
        self.windows.push(window);
    }

    /// Adds the given [ScopeLegend] to the open scopes. The scope will be last
    /// one in the list, meaning it will be displayed on the bottom. The user
    /// can reorder them afterwards if desired.
    ///
    /// Note: The scope might appear outside of the visible area of the main
    /// scrollable. It might be necessary to automatically scroll there to avoid
    /// confusion.
    fn push_scope(&mut self, scope: ScopeLegend) {
        let plotters = self
            .windows
            .iter()
            .map(|_| scope.create_plotter())
            .collect();
        self.plotters.push_row(plotters);
        self.scopes.push(scope);
    }

    // TODO: reorder windows

    // TODO: reorder scopes

    // TODO: close window

    // TODO: close scope
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::state::{Run, Source};

    use super::TimescapeViewer;

    fn app_with_one_run() -> TimescapeViewer {
        let mut viewer = TimescapeViewer::default();
        viewer.sources.push(Source::new_example_sines());
        viewer
    }

    #[test]
    fn test_push_window() {
        // Arrange
        let mut viewer = app_with_one_run();
        let run = Rc::new(Run::new_example_sines());

        // Act
        viewer.push_window(run);

        // Assert
        assert_eq!(
            viewer.windows.len(),
            2,
            "Expected two windows, the old and the newly added one.",
        );
        assert_eq!(
            viewer.plotters.cols(),
            1,
            "Expected two column of plotters, one per window.",
        );
    }
}
