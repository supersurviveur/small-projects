use crevice::std430::AsStd430;
use wgpu::{
    BindGroup, Buffer, CommandEncoder, Device, PushConstantRange, RenderPipeline, ShaderStages,
    SurfaceConfiguration, TextureView, include_wgsl,
};

use crate::render::msaa::{SAMPLE_COUNT, create_msaa_texture};

pub struct Pipeline {
    pub texture: TextureView,
    pub pipeline: RenderPipeline,
    pub bind_group: BindGroup,
}

impl Pipeline {
    pub fn new(device: &Device, config: &SurfaceConfiguration, buffer: &Buffer) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../../shaders/shader.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Points bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Points bg"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[PushConstantRange {
                stages: ShaderStages::VERTEX,
                range: 0..PushConstants::std430_size_static() as u32,
            }],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let texture = create_msaa_texture(
            device,
            config.format,
            config.width,
            config.height,
            SAMPLE_COUNT,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );

        Self {
            pipeline,
            texture,
            bind_group,
        }
    }
    pub fn render_pass(
        &mut self,
        encoder: &mut CommandEncoder,
        config: &SurfaceConfiguration,
        resolve_texture: &TextureView,
        buffer: &Buffer,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture,
                resolve_target: Some(resolve_texture),
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_push_constants(
            ShaderStages::VERTEX,
            0,
            PushConstants {
                width: config.width,
                height: config.height,
            }
            .as_std430()
            .as_bytes(),
        );
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
    pub fn resize(&mut self, device: &Device, config: &SurfaceConfiguration, buffer: &Buffer) {
        self.texture = create_msaa_texture(
            device,
            config.format,
            config.width,
            config.height,
            SAMPLE_COUNT,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Points bg"),
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
    }
}

#[repr(C)]
#[derive(Clone, Copy, AsStd430)]
pub struct PushConstants {
    width: u32,
    height: u32,
}
