//! A GPU-accelerated trace/chart background grid renderer based on
//! https://observablehq.com/@rreusser/locally-scaled-domain-coloring-part-1-contour-plots
pub mod pipeline;

use glam::Vec4;
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
        _viewport: &shader::Viewport,
    ) {
        // Upload data to GPU
        pipeline.update(
            queue,
            &Uniforms {
                background: Vec4::new(0.0 / 255.0, 13.0 / 255.0, 24.0 / 255.0, 1.0),
                antialias_width: 1.0,
                octave_divisions: 5.0,
                baseline_spacing: 5.0,
                ramp_power: 2.45,
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
