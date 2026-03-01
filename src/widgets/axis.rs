use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Widget};
use iced::{Length, Rectangle, Size};

pub enum Scaling {
    Linear,
    Logarithmic,
}

pub struct Axis {
    minimum: f64,
    maximum: f64,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Axis
where
    Renderer: iced::advanced::renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size {
            width: 80.0,
            height: 16.0,
        })
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        _layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        todo!()
    }
}
