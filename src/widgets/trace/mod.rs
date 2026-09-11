//! A GPU-accelerated trace/chart renderer backed by Implicit In-order Forests with range queries
//! performed in the vertex shader.
pub mod pipeline;

use iced::widget::shader;

use pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct Trace {
    width: u32,
}

impl<Message> shader::Program<Message> for Trace {
    type State = ();

    type Primitive = Primitive;

    fn draw(
        &self,
        state: &Self::State,
        cursor: iced_core::mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        todo!()
    }
}

#[derive(Debug)]
pub struct Primitive {

}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &iced::wgpu::Device,
        queue: &iced::wgpu::Queue,
        bounds: &iced::Rectangle,
        viewport: &shader::Viewport,
    ) {
        // Upload data to GPU
        pipeline.update(
            device,
            queue,
            viewport.physical_size(),
            // TODO
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
