//! A GPU-accelerated trace/chart renderer backed by Implicit In-order Forests with range queries
//! performed in the vertex shader.
pub mod mesh;
pub mod pipeline;

use iced::{Color, widget::shader};
use std::ops::Range;

use pipeline::{Pipeline, Uniforms};

use crate::core;
use crate::utilities::ColorExt;

#[derive(Debug, Clone, PartialEq)]
pub struct Trace<'a> {
    time: Range<i64>,
    values: Range<f32>,
    trace: &'a core::Trace,
    color: Color,
}

impl<'a> Trace<'a> {
    pub fn new(time: Range<i64>, values: Range<f32>, trace: &'a core::Trace, color: Color) -> Self {
        Self {
            time,
            values,
            trace,
            color,
        }
    }
}

impl<Message> shader::Program<Message> for Trace<'_> {
    type State = ();

    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced_core::mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        Self::Primitive {
            uniforms: Uniforms {
                line_color: self.color.to_vec4(),
                spread_alpha: 0.2,
                antialias_width: 1.0,
                stroke: 1.0,
                min_value: 0.0,
                max_value: 1.0,
                begin: 0.0,
                end: 1.0,
                range: (1.0 - 0.0) / bounds.width,
            },
        }
    }
}

#[derive(Debug)]
pub struct Primitive {
    uniforms: Uniforms,
}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &iced::wgpu::Device,
        queue: &iced::wgpu::Queue,
        _bounds: &iced::Rectangle,
        viewport: &shader::Viewport,
    ) {
        // Upload data to GPU
        pipeline.update(device, queue, viewport.physical_size(), &self.uniforms);
    }

    fn render(
        &self,
        pipeline: &Pipeline,
        encoder: &mut iced::wgpu::CommandEncoder,
        target: &iced::wgpu::TextureView,
        clip_bounds: &iced::Rectangle<u32>,
    ) {
        // Render primitive
        pipeline.render(target, encoder, *clip_bounds);
    }
}
