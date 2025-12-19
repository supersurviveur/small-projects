use std::sync::Arc;

use wgpu::{BufferDescriptor, BufferUsages};
use winit::window::Window;

use crate::render::{
    msaa::SAMPLE_COUNT,
    pipelines::{Pipeline, PushConstants},
    state::State,
};

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::PUSH_CONSTANTS | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits {
                        max_push_constant_size: size_of::<PushConstants>() as u32,
                        ..Default::default()
                    }
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let size = ((config.width + 1) * (config.height + 1)) as u64 * size_of::<f32>() as u64;
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Height buffer"),
            size,
            usage: BufferUsages::COPY_DST | BufferUsages::VERTEX | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let pipeline = Pipeline::new(&device, &config, &buffer);

        let noise = noise::Perlin::new(3);

        let mut state = Self {
            pipeline,

            surface,
            device,
            queue,
            height_points: vec![1.; ((config.width + 1) * (config.height + 1)) as usize],
            config,
            is_surface_configured: false,
            window,
            sample_count: SAMPLE_COUNT,
            buffer,
            noise,
            z: 0.,
        };

        state.update_height_points();

        Ok(state)
    }
}
