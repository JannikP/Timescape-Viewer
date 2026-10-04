use glam::Vec4;
use iced::widget::shader;
use iced::{Rectangle, Size, wgpu};
use std::borrow::Cow;
use std::collections::HashMap;

use super::mesh::Mesh;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub line_color: Vec4,
    pub spread_alpha: f32,
    pub antialias_width: f32,
    pub stroke: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub begin: f32,
    pub end: f32,
    pub range: f32,
}

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    mesh_cache: HashMap<u32, Mesh>,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("TraceShader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("trace.wgsl"))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("TraceShaderPipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Mesh::desc(),
                    // TODO: Trace data buffer descriptor here?
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            multiview: None,
            cache: None,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("TraceUniformBuffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = pipeline.get_bind_group_layout(0);
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("TraceUniformBindGroup"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            mesh_cache: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_size: Size<u32>,
        uniforms: &Uniforms,
    ) {
        let mesh = self
            .mesh_cache
            .entry(target_size.width)
            .or_insert_with(|| Mesh::create(device, target_size.width));
        mesh.mark_used();

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let key = viewport.width; // FIXME: Ugly workaround for testing. Should be stored in the primitive instead.
        let mesh = self
            .mesh_cache
            .get(&key)
            .expect("The mesh should have been created in the update step.");
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("TraceRenderPass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_viewport(
            viewport.x as f32,
            viewport.y as f32,
            viewport.width as f32,
            viewport.height as f32,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_vertex_buffer(0, mesh.vertex_buffer_slice());
        pass.set_index_buffer(mesh.index_buffer_slice(), mesh.index_format());
        pass.draw_indexed(mesh.index_range(), 0, 0..1);
    }
}

impl shader::Pipeline for Pipeline {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Pipeline {
        Self::new(device, queue, format)
    }

    fn trim(&mut self) {
        self.mesh_cache.retain(|_, mesh| mesh.is_used());
        self.mesh_cache
            .iter_mut()
            .for_each(|(_, mesh)| mesh.clear_usage());
    }
}
