use std::ops::Div;

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

const LINE_THICKNESS: f32 = 2.0;

#[derive(Debug, Clone, Default)]
pub struct Chart {
    spread_mesh: Option<Mesh>,
    line_mesh: Option<Mesh>,
    samples: usize,
}

pub fn chart() -> Chart {
    Chart::default()
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
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let samples = bounds.width.div(2.0).floor() as usize;
        if samples != self.samples || self.line_mesh.is_none() || self.spread_mesh.is_none() {
            let (min_values, avg_values, max_values) =
                generate_random_example_data(samples, 0.0, 100.0);
            let (spread_mesh, line_mesh) = build_mesh(
                min_values.as_slice(),
                avg_values.as_slice(),
                max_values.as_slice(),
                bounds.width,
                bounds.height,
                0.0,
                100.0,
                &[0.87218969, 0.95960708, 0.89725384, 1.0],
                0.15,
            );
            self.spread_mesh = Some(spread_mesh);
            self.line_mesh = Some(line_mesh);
            self.samples = samples;
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
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if self.line_mesh.is_some() && self.spread_mesh.is_some() {
            renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
                renderer.draw_mesh(self.spread_mesh.clone().unwrap());
                renderer.draw_mesh(self.line_mesh.clone().unwrap());
            });
        }
    }
}

impl<Message, Theme, Renderer> From<Chart> for Element<'_, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer + MeshRenderer,
{
    fn from(chart: Chart) -> Self {
        Self::new(chart)
    }
}

fn generate_random_example_data(
    samples: usize,
    min_val: f64,
    max_val: f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    use rand;
    use rand::RngExt;

    let mut rng = rand::rng();
    let mut min = Vec::with_capacity(samples);
    let mut avg = Vec::with_capacity(samples);
    let mut max = Vec::with_capacity(samples);
    let spread = (max_val - min_val) * 0.8;
    let offset = spread * 0.1 + min_val;
    let std = spread * 0.35;
    for _ in 0..samples {
        let v = rng.random_range(0.0..spread) + offset;
        let up = v + rng.random_range(0.0..std);
        let down = v - rng.random_range(0.0..std);
        min.push(down);
        avg.push(v);
        max.push(up);
    }
    (min, avg, max)
}

fn physical_to_pixel(value: f64, minimum: f64, range: f64, height: f32) -> f32 {
    let fraction = (value - minimum) / range;
    (1.0 - fraction as f32) * height
}

fn build_mesh(
    min_values: &[f64],
    avg_values: &[f64],
    max_values: &[f64],
    width: f32,
    height: f32,
    minimum: f64,
    maximum: f64,
    color: &[f32; 4],
    spread_alpha: f32,
) -> (Mesh, Mesh) {
    let samples = min_values.len();
    assert!(avg_values.len() == samples);
    assert!(max_values.len() == samples);

    // Signal spread (the transparent background area)
    let mut spread_vertices = Vec::with_capacity(samples * 2);
    let mut spread_indices = Vec::with_capacity(samples * 6);
    let spread_color = color::pack([color[0], color[1], color[2], spread_alpha]);
    // Signal line (the opaque foreground line)
    let mut line_vertices = Vec::with_capacity(samples * 2);
    let mut line_indices = Vec::with_capacity(samples * 6);
    let line_color = color::pack(*color);

    let range = maximum - minimum;
    let step = 2.0 * width / samples as f32;

    for i in 0..samples {
        let x = i as f32 * step;

        // Signal spread (the transparent background area)
        spread_vertices.push(SolidVertex2D {
            position: [x, physical_to_pixel(max_values[i], minimum, range, height)],
            color: spread_color,
        });
        spread_vertices.push(SolidVertex2D {
            position: [x, physical_to_pixel(min_values[i], minimum, range, height)],
            color: spread_color,
        });

        // Signal line (the opaque foreground line)
        let y = physical_to_pixel(avg_values[i], minimum, range, height);
        line_vertices.push(SolidVertex2D {
            position: [x, y - 0.5 * LINE_THICKNESS],
            color: line_color,
        });
        line_vertices.push(SolidVertex2D {
            position: [x, y + 0.5 * LINE_THICKNESS],
            color: line_color,
        });
    }

    for i in 1..samples {
        // 0---2
        // |  /|
        // |/  |
        // 1---3
        let i_n = i as u32 * 2;
        let i_p = i_n - 2;

        // Signal spread (the transparent background area)
        spread_indices.push(i_p); // 0
        spread_indices.push(i_n); // 2
        spread_indices.push(i_p + 1); // 1
        spread_indices.push(i_n); // 2
        spread_indices.push(i_n + 1); // 3
        spread_indices.push(i_p + 1); // 1

        // Signal line (the opaque foreground line)
        line_indices.push(i_p); // 0
        line_indices.push(i_n); // 2
        line_indices.push(i_p + 1); // 1
        line_indices.push(i_n); // 2
        line_indices.push(i_n + 1); // 3
        line_indices.push(i_p + 1); // 1
    }

    let clip = Rectangle {
        x: 0.0,
        y: 0.0,
        width,
        height,
    };

    (
        Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: spread_vertices,
                indices: spread_indices,
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: clip,
        },
        Mesh::Solid {
            buffers: mesh::Indexed {
                vertices: line_vertices,
                indices: line_indices,
            },
            transformation: Transformation::IDENTITY,
            clip_bounds: clip,
        },
    )
}
