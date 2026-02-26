//! A draggable divider between scopes or windows. Dragging it allows the user to resize the
//! relative sizes of the scopes or windows.

use iced_core as core;
use iced_core::layout;
use iced_core::mouse;
use iced_core::renderer;
use iced_core::widget::Tree;
use iced_core::{Element, Layout, Length, Rectangle, Size, Widget};

/// Creates a new [`Divider`] widget that fills the available horizontal space and can be dragged
/// vertically to adjust the height of the neighboring widgets.
pub fn horizontal_divider() -> Divider {
    Divider::new(Direction::Horizontal).width(Length::Fill)
}

/// Creates a new [`Divider`] widget that fills the available vertical space and can be dragged
/// horizontally to adjust the height of the neighboring widgets.
pub fn vertical_divider() -> Divider {
    Divider::new(Direction::Vertical).height(Length::Fill)
}

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
#[derive(Debug)]
pub struct Divider {
    direction: Direction,
    width: Length,
    height: Length,
}

impl Divider {
    /// Creates some empty [`Divider`] with no size.
    pub fn new(direction: Direction) -> Self {
        Divider {
            direction,
            width: Length::Shrink,
            height: Length::Shrink,
        }
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

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Divider
where
    Renderer: core::Renderer,
{
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
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over {
            match self.direction {
                Direction::Horizontal => mouse::Interaction::ResizingVertically,
                Direction::Vertical => mouse::Interaction::ResizingHorizontally,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Divider> for Element<'a, Message, Theme, Renderer>
where
    Renderer: core::Renderer,
    Message: 'a,
{
    fn from(space: Divider) -> Element<'a, Message, Theme, Renderer> {
        Element::new(space)
    }
}
