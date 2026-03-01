//! A draggable divider between scopes or windows. Dragging it allows the user to resize the
//! relative sizes of the scopes or windows.

use iced::touch;
use iced::{Event, Point};
use iced_core as core;
use iced_core::Clipboard;
use iced_core::Shell;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::tree::{self, Tree};
use iced_core::window;
use iced_core::{Element, Layout, Length, Rectangle, Size, Widget};

use crate::constants::layout::PANEL_GAP;

/// The direction of [`Divider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Vertical division, horizontal resizing.
    Vertical,
    /// Horizontal division, vertical resizing.
    Horizontal,
}

/// A draggable divider between scopes or windows.
///
/// Dragging it allows the user to resize the relative sizes of the scopes or windows.
pub struct Divider<'a, Message> {
    direction: Direction,
    width: Length,
    height: Length,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
    on_release: Option<Message>,
    status: Option<Status>,
}

impl<'a, Message> Divider<'a, Message> {
    /// The default thickness of a [`Divider`].
    pub const DEFAULT_THICKNESS: u32 = PANEL_GAP;

    /// Creates a new [`Divider`].
    ///
    /// It expects:
    ///   * the direction of the [`Divider`]. It moves perpendicular to it.
    ///   * the current value of the [`Divider`]
    ///   * a function that will be called when the [`Divider`] is dragged.
    ///     It receives the new value of the [`Divider`] and must produce a
    ///     `Message`.
    pub fn new<F>(direction: Direction, value: f32, on_change: F) -> Self
    where
        F: 'a + Fn(f32) -> Message,
    {
        let width = match direction {
            Direction::Horizontal => Length::Fill,
            Direction::Vertical => Self::DEFAULT_THICKNESS.into(),
        };
        let height = match direction {
            Direction::Horizontal => Self::DEFAULT_THICKNESS.into(),
            Direction::Vertical => Length::Fill,
        };
        Divider {
            direction,
            width,
            height,
            value,
            on_change: Box::new(on_change),
            on_release: None,
            status: None,
        }
    }

    /// Creates a new horizontal [`Divider`] that moves in vertical direction.
    ///
    /// It expects:
    ///   * the current value of the [`Divider`]
    ///   * a function that will be called when the [`Divider`] is dragged.
    ///     It receives the new value of the [`Divider`] and must produce a
    ///     `Message`.
    pub fn horizontal<F>(value: f32, on_change: F) -> Self
    where
        F: 'a + Fn(f32) -> Message,
    {
        Self::new(Direction::Horizontal, value, on_change)
    }

    /// Creates a new vertical [`Divider`] that moves in horizontal direction.
    ///
    /// It expects:
    ///   * the current value of the [`Divider`]
    ///   * a function that will be called when the [`Divider`] is dragged.
    ///     It receives the new value of the [`Divider`] and must produce a
    ///     `Message`.
    pub fn vertical<F>(value: f32, on_change: F) -> Self
    where
        F: 'a + Fn(f32) -> Message,
    {
        Self::new(Direction::Vertical, value, on_change)
    }

    /// Sets the width of the [`Divider`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Divider`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Divider<'a, Message>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        let mut update = || {
            let current_value = self.value;

            let locate = |cursor_position: Point| -> Option<f32> {
                let bounds = layout.bounds();
                let delta = match self.direction {
                    Direction::Horizontal => cursor_position.y - bounds.y,
                    Direction::Vertical => cursor_position.x - bounds.x,
                };
                let new_value = current_value + delta;
                Some(new_value)
            };

            let change = |new_value: f32| {
                if (self.value - new_value).abs() > f32::EPSILON {
                    shell.publish((self.on_change)(new_value));

                    self.value = new_value;
                }
            };

            match &event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if let Some(cursor_position) = cursor.position_over(layout.bounds()) {
                        let _ = locate(cursor_position).map(change);
                        state.is_dragging = true;
                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. }) => {
                    if state.is_dragging {
                        if let Some(on_release) = self.on_release.clone() {
                            shell.publish(on_release);
                        }
                        state.is_dragging = false;
                    }
                }
                Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Touch(touch::Event::FingerMoved { .. }) => {
                    if state.is_dragging {
                        let _ = cursor.land().position().and_then(locate).map(change);

                        shell.capture_event();
                    }
                }
                _ => {}
            }
        };

        update();

        let current_status = if state.is_dragging {
            Status::Dragged
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over || state.is_dragging {
            match self.direction {
                Direction::Horizontal => mouse::Interaction::ResizingVertically,
                Direction::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Divider<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
    Message: 'a + Clone,
{
    fn from(divider: Divider<'a, Message>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(divider)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_dragging: bool,
}

/// The possible status of a [`Divider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Divider`] can be interacted with.
    Active,
    /// The [`Divider`] is being hovered.
    Hovered,
    /// The [`Divider`] is being dragged.
    Dragged,
}
