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
use iced::window::settings::Settings as WindowSettings;
use iced::{Element, Font, Settings, Task, Theme};
use log::{debug, info};
use rust_i18n::{i18n, t};

// use commands::choose_file::choose_file;
use logging::setup_logger;
use messages::Message;
use state::{Modal, ScopeLegend, ScopePlotter, Stage, Window};
use views::{modal, view_backstage, view_timescape};

use crate::constants::icons::app_icon;
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
        vsync: false,
    })
    .window(WindowSettings {
        icon: app_icon(),
        ..Default::default()
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
                // Task::perform(choose_file(), |maybe_origin| match maybe_origin {
                //     Some(origin) => Message::Open(origin),
                //     None => Message::None,
                // })

                // TODO: Open a dummy source while developing the UI. Remove later.
                self.push_source(Source::new_example_sines());
                Task::none()
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
            Message::AddLineChart => {
                self.push_scope(ScopeLegend::line_chart("440 Hz sine"));
                Task::none()
            }
            Message::AddSpectrogram => {
                self.push_scope(ScopeLegend::spectrogram("440 Hz sine"));
                Task::none()
            }
            Message::AddTrailChart => {
                self.push_scope(ScopeLegend::trail_chart("440 Hz sine"));
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

    /// Adds a new source to the app. If there is no window yet, open one with
    /// the full duration of the source's first run.
    fn push_source(&mut self, source: Source) {
        let first_run = source.first_run();
        self.sources.push(source);
        if self.windows.is_empty() && first_run.is_some() {
            self.push_window(first_run.unwrap()); // Checked above.
        }
    }

    /// Adds a new window for the given [Run]. The window will be last one in
    /// the list, meaning it will be displayed on the right. The user can
    /// reorder them afterwards if desired.
    fn push_window(&mut self, run: Rc<Run>) {
        let window = Window::new(run);
        let plotters: Vec<ScopePlotter> = self
            .scopes
            .iter()
            .map(|scope| scope.create_plotter())
            .collect();
        if !plotters.is_empty() {
            self.plotters.push_col(plotters);
        }
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
        let plotters: Vec<ScopePlotter> = self
            .windows
            .iter()
            .map(|_| scope.create_plotter())
            .collect();
        if !plotters.is_empty() {
            self.plotters.push_row(plotters);
        }
        self.scopes.push(scope);
    }

    // TODO: reorder windows

    // TODO: reorder scopes

    /// Closes the window at the given index, removing it from the list of
    /// windows and all plotters that belong to it.
    fn close_window(&mut self, index: usize) {
        if index < 1 || index >= self.windows.len() {
            return;
        }
        self.windows.remove(index);
        self.plotters.remove_col(index);
    }

    // TODO: close scope
}

#[cfg(test)]
mod tests {
    use crate::state::{ScopeLegend, Source};

    use super::TimescapeViewer;

    fn app_with_one_run() -> TimescapeViewer {
        let mut app = TimescapeViewer::default();
        app.push_source(Source::new_example_sines());
        app.push_scope(ScopeLegend::line_chart("440 Hz sine"));
        app
    }

    #[test]
    fn test_push_window() {
        // Arrange
        let mut app = app_with_one_run();
        let run = app
            .sources
            .first()
            .map(Source::first_run)
            .flatten()
            .expect("There should be one run added by `app_with_one_run`.");

        // Act
        app.push_window(run);

        // Assert
        assert_eq!(
            app.windows.len(),
            2,
            "Expected two windows, the old and the newly added one.",
        );
        assert_eq!(
            app.plotters.cols(),
            2,
            "Expected two column of plotters, one per window.",
        );
    }

    #[test]
    fn test_push_scope() {
        // Arrange
        let mut app = app_with_one_run();

        // Act
        app.push_scope(ScopeLegend::line_chart("440 Hz sine"));

        // Assert
        assert_eq!(
            app.scopes.len(),
            2,
            "Expected two scopes, the old and the newly added one.",
        );
        assert_eq!(
            app.plotters.rows(),
            2,
            "Expected two row of plotters, one per scope.",
        )
    }

    #[test]
    fn test_close_second_window() {
        // Arrange
        let mut app = app_with_one_run();
        let run = app
            .sources
            .first()
            .map(Source::first_run)
            .flatten()
            .expect("There should be one run added by `app_with_one_run`.");
        app.push_window(run);

        // Act
        app.close_window(1); // Second window

        // Assert
        assert_eq!(
            app.windows.len(),
            1,
            "Expected one window, the newly added one should be closed.",
        );
        assert_eq!(
            app.plotters.cols(),
            1,
            "Expected one column of plotters, one for the remaining window.",
        );
    }

    #[test]
    fn test_close_only_window() {
        // Arrange
        let mut app = app_with_one_run();

        // Act
        app.close_window(0); // Only window

        // Assert
        assert_eq!(
            app.windows.len(),
            1,
            "Expected one window, the only window can't be closed.",
        );
        assert_eq!(
            app.plotters.cols(),
            1,
            "Expected one column of plotters, one for the remaining window.",
        );
    }
}
