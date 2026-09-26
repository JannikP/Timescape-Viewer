//! A GPU-accelerated trace/chart background grid renderer based on
//! https://observablehq.com/@rreusser/locally-scaled-domain-coloring-part-1-contour-plots
pub mod pipeline;

use glam::Vec2;
use iced::widget::shader;

use pipeline::{Pipeline, Uniforms};

#[derive(Debug, Clone)]
pub struct Grid { }

impl Grid {
    pub fn new() -> Self {
        Self { }
    }
}

impl<Message> shader::Program<Message> for Grid {
    type State = ();

    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced_core::mouse::Cursor,
        _bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Self::Primitive { }
    }
}

#[derive(Debug)]
pub struct Primitive { }

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        _device: &iced::wgpu::Device,
        queue: &iced::wgpu::Queue,
        _bounds: &iced::Rectangle,
        viewport: &shader::Viewport,
    ) {
        // Upload data to GPU
        pipeline.update(
            queue,
            &Uniforms {
                resolution: Vec2::new(viewport.physical_width() as f32, viewport.physical_height() as f32),
                center: Vec2::new(-1.5, 0.0),
                scale: 1.0 / 800.0,
                max_iter: 20,
            },
        );
    }

    fn render(
        &self,
        pipeline: &Pipeline,
        encoder: &mut iced::wgpu::CommandEncoder,
        target: &iced::wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        // Render primitive
        pipeline.render(
            target,
            encoder,
            *clip_bounds,
            // TODO
        );
    }
}
