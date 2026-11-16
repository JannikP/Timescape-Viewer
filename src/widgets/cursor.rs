use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Tree, Widget};
use iced::mouse::{self, Button};
use iced::{Color, Element, Event, Length, Point, Rectangle, Size};

use crate::core::CursorId;
use crate::messages::{Message, window};
use crate::state::Window;

/// The distance in logical pixels within which a press is considered to hit an
/// existing cursor line. Pressing on a cursor line dismisses it instead of
/// placing a new cursor.
pub const HIT_TOLERANCE: f32 = 4.0;

/// Widget that overlays a vertical line for the primary and secondary cursors
/// of a [`Window`] on top of a plot and lets the user place, move and dismiss
/// them.
///
/// Interaction:
/// * A left click places the primary cursor at the clicked position. While the
///   button is held down the cursor follows the mouse. Pressing on the primary
///   cursor line dismisses it.
/// * A right click does the same for the secondary cursor.
///
/// The widget fills the area of the plot it is stacked on and maps the span of
/// the given window onto its width. Cursors outside the visible span are not
/// drawn.
pub struct Cursors<'a, Theme>
where
    Theme: Catalog,
{
    window: &'a Window,
    width: Length,
    height: Length,
    class: Theme::Class<'a>,
}

impl<'a, Theme> Cursors<'a, Theme>
where
    Theme: Catalog,
{
    /// Creates a [`Cursors`] overlay displaying the cursors of the given
    /// [`Window`].
    #[must_use]
    pub fn new(window: &'a Window) -> Self {
        Self {
            window,
            width: Length::Fill,
            height: Length::Fill,
            class: Theme::default(),
        }
    }

    /// Sets the width of the [`Cursors`] overlay.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Cursors`] overlay.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the style of the [`Cursors`] overlay.
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }
}

/// The widget state of a [`Cursors`] overlay which is preserved even if the
/// view rebuilds the element.
#[derive(Debug, Default, Clone, Copy)]
struct DragState {
    dragging: Option<CursorId>,
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for Cursors<'a, Theme>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        iced::advanced::widget::tree::Tag::of::<DragState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(DragState::default())
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
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<DragState>();
        let bounds = layout.bounds();
        let Some(position) = cursor.position_in(bounds) else {
            if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
                state.dragging = None;
            }
            return;
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(button)) => {
                let side = match button {
                    Button::Left => Side::Primary,
                    Button::Right => Side::Secondary,
                    _ => return,
                };
                if let Some(message) =
                    press_cursor(self.window, bounds, position, side)
                {
                    state.dragging = pressed_draggable(&message);
                    shell.publish(message);
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(_)) => {
                if state.dragging.take().is_some() {
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(kind) = state.dragging {
                    let timestamp = timestamp_at(self.window, bounds, position);
                    if self.window.span().contains(timestamp) {
                        shell.publish(place_message(self.window.index(), kind, timestamp));
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);
        if let Some(timestamp) = self.window.primary_cursor()
            && let Some(x) = pixel_of(self.window, bounds, timestamp)
        {
            draw_line(renderer, bounds, x, style.primary);
        }
        if let Some(timestamp) = self.window.secondary_cursor()
            && let Some(x) = pixel_of(self.window, bounds, timestamp)
        {
            draw_line(renderer, bounds, x, style.secondary);
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        match cursor.position_in(layout.bounds()) {
            Some(position) if hit_cursor(self.window, layout.bounds(), position).is_some() => {
                mouse::Interaction::Pointer
            }
            _ => mouse::Interaction::None,
        }
    }
}

/// The cursor line a press belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Primary,
    Secondary,
}

/// Handles a button press and returns the message to be published.
///
/// Pressing on an existing cursor line dismisses it. Otherwise a cursor of the
/// pressed [`Side`] is placed at the pressed position, if that position is
/// within the window's span.
fn press_cursor(window: &Window, bounds: Rectangle, position: Point, side: Side) -> Option<Message> {
    if let Some(id) = hit_cursor(window, bounds, position) {
        return Some(Message::Window(
            window.index(),
            window::Message::Dismiss(id),
        ));
    }
    let kind = match side {
        Side::Primary => CursorId::Primary,
        Side::Secondary => CursorId::Secondary,
    };
    let timestamp = timestamp_at(window, bounds, position);
    if window.span().contains(timestamp) {
        Some(place_message(window.index(), kind, timestamp))
    } else {
        None
    }
}

/// Only a freshly placed cursor can be dragged. Dismissing one ends any drag.
fn pressed_draggable(message: &Message) -> Option<CursorId> {
    match message {
        Message::Window(_, window::Message::Place { cursor, .. }) => Some(*cursor),
        _ => None,
    }
}

/// Returns the [`CursorId`] whose line is within [`HIT_TOLERANCE`] pixels of
/// `position`. If both lines are close, the primary cursor wins.
fn hit_cursor(window: &Window, bounds: Rectangle, position: Point) -> Option<CursorId> {
    let hits = |timestamp: Option<i64>| {
        timestamp
            .and_then(|t| pixel_of(window, bounds, t))
            .is_some_and(|x| (x - position.x).abs() <= HIT_TOLERANCE)
    };
    if hits(window.primary_cursor()) {
        Some(CursorId::Primary)
    } else if hits(window.secondary_cursor()) {
        Some(CursorId::Secondary)
    } else {
        None
    }
}

fn place_message(index: usize, kind: CursorId, timestamp: i64) -> Message {
    Message::Window(
        index,
        window::Message::Place {
            cursor: kind,
            position: timestamp,
        },
    )
}

/// Maps the x coordinate of `position` (within `bounds`) to the timestamp of
/// the window's span displayed at that pixel.
fn timestamp_at(window: &Window, bounds: Rectangle, position: Point) -> i64 {
    let span = window.span();
    let fraction = (position.x / bounds.width).clamp(0.0, 1.0) as f64;
    span.begin + ((span.end - span.begin) as f64 * fraction).round() as i64
}

/// Maps a `timestamp` of the window's span to the x coordinate of its vertical
/// line within `bounds`. Returns [`None`] if the timestamp is outside the
/// visible span.
fn pixel_of(window: &Window, bounds: Rectangle, timestamp: i64) -> Option<f32> {
    let span = window.span();
    if !span.contains(timestamp) {
        return None;
    }
    let fraction = (timestamp - span.begin) as f64 / (span.end - span.begin) as f64;
    Some(bounds.x + bounds.width * fraction as f32)
}

fn draw_line<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    x: f32,
    color: Color,
) {
    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: x - 0.5,
                y: bounds.y,
                width: 1.0,
                height: bounds.height,
            },
            snap: false,
            ..renderer::Quad::default()
        },
        color,
    );
}

impl<'a, Theme, Renderer> From<Cursors<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(cursors: Cursors<'a, Theme>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(cursors)
    }
}

/// The style of a [`Cursors`] overlay.
///
/// If not specified with [`Cursors::style`] the theme will provide the style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The line [`iced::Color`] of the primary cursor.
    pub primary: Color,
    /// The line [`iced::Color`] of the secondary cursor.
    pub secondary: Color,
}

/// A styling function for a [`Cursors`] overlay.
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

/// The theme catalog of a [`Cursors`] overlay.
///
/// All themes that can be used with [`Cursors`] must implement this trait.
///
/// Although, in order to use [`Cursors::style`] with `MyTheme`,
/// [`Catalog::Class`] must implement `From<StyleFn<'_, MyTheme>>`.
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
