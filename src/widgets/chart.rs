use iced::advanced::Shell;
use iced::advanced::graphics::color;
use iced::advanced::graphics::mesh::Renderer as MeshRenderer;
use iced::advanced::graphics::mesh::{self, Mesh, SolidVertex2D};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Element, Event, Length, Rectangle, Size, Transformation, Vector};
use iced_core::Clipboard;
use iced_core::widget::Tree;

#[derive(Debug, Clone, Copy, Default)]
pub struct Chart;

pub fn chart() -> Chart {
    Chart
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Chart
where
    Renderer: iced_core::Renderer + MeshRenderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let width = limits.max().width;
        let height = limits.max().height;

        layout::Node::new(Size::new(width, height))
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if cursor.is_over(layout.bounds()) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let a = 0.5;

        // R O Y G B I V
        let color_r = [1.0, 0.0, 0.0, a];
        let color_o = [1.0, 0.5, 0.0, a];
        let color_y = [1.0, 1.0, 0.0, a];
        let color_g = [0.0, 1.0, 0.0, a];
        let color_gb = [0.0, 1.0, 0.5, a];
        let color_b = [0.0, 0.2, 1.0, a];
        let color_i = [0.5, 0.0, 1.0, a];
        let color_v = [0.75, 0.0, 0.5, a];

        let posn_center = {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                [cursor_position.x, cursor_position.y]
            } else {
                [bounds.width / 2.0, bounds.height / 2.0]
            }
        };

        let posn_tl = [0.0, 0.0];
        let posn_t = [bounds.width / 2.0, 0.0];
        let posn_tr = [bounds.width, 0.0];
        let posn_r = [bounds.width, bounds.height / 2.0];
        let posn_br = [bounds.width, bounds.height];
        let posn_b = [(bounds.width / 2.0), bounds.height];
        let posn_bl = [0.0, bounds.height];
        let posn_l = [0.0, bounds.height / 2.0];

        let mesh = Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: vec![
                    SolidVertex2D {
                        position: posn_center,
                        color: color::pack([1.0, 1.0, 1.0, 0.0]),
                    },
                    SolidVertex2D {
                        position: posn_tl,
                        color: color::pack(color_r),
                    },
                    SolidVertex2D {
                        position: posn_t,
                        color: color::pack(color_o),
                    },
                    SolidVertex2D {
                        position: posn_tr,
                        color: color::pack(color_y),
                    },
                    SolidVertex2D {
                        position: posn_r,
                        color: color::pack(color_g),
                    },
                    SolidVertex2D {
                        position: posn_br,
                        color: color::pack(color_gb),
                    },
                    SolidVertex2D {
                        position: posn_b,
                        color: color::pack(color_b),
                    },
                    SolidVertex2D {
                        position: posn_bl,
                        color: color::pack(color_i),
                    },
                    SolidVertex2D {
                        position: posn_l,
                        color: color::pack(color_v),
                    },
                ],
                indices: vec![
                    0, 1, 2, // TL
                    0, 2, 3, // T
                    0, 3, 4, // TR
                    0, 4, 5, // R
                    0, 5, 6, // BR
                    0, 6, 7, // B
                    0, 7, 8, // BL
                    0, 8, 1, // L
                ],
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: Rectangle::INFINITE,
        };

        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            renderer.draw_mesh(mesh);
        });
    }
}

impl<Message, Theme, Renderer> From<Chart> for Element<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer + MeshRenderer,
{
    fn from(rainbow: Chart) -> Self {
        Self::new(rainbow)
    }
}
