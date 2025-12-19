use noise::NoiseFn;
use std::sync::Arc;

use wgpu::{Buffer, BufferDescriptor, BufferUsages};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::render::pipelines::Pipeline;

pub mod init;

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub pipeline: Pipeline,
    pub window: Arc<Window>,

    pub sample_count: u32,

    pub height_points: Vec<f32>,
    pub buffer: Buffer,
    pub noise: noise::Perlin,
    pub z: f64,
}

impl State {
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;

            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            self.height_points
                .resize(((width + 1) * (height + 1)) as usize, 0.);

            let size = ((width + 1) * (height + 1)) as u64 * size_of::<f32>() as u64;
            self.buffer = self.device.create_buffer(&BufferDescriptor {
                label: Some("Height buffer"),
                size,
                usage: BufferUsages::COPY_DST | BufferUsages::VERTEX | BufferUsages::STORAGE,
                mapped_at_creation: false,
            });

            self.pipeline
                .resize(&self.device, &self.config, &self.buffer);
        }
    }
    pub fn update(&mut self) {
        self.update_height_points();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.pipeline
            .render_pass(&mut encoder, &self.config, &view, &self.buffer);
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }
    pub fn update_height_points(&mut self) {
        self.z += 0.001;

        for x in 0..self.config.width + 1 {
            for y in 0..self.config.height + 1 {
                self.height_points[(y * (self.config.width + 1) + x) as usize] = self.noise.get([
                    x as f64 / self.config.width as f64 * 10.,
                    y as f64 / self.config.height as f64 * 10.,
                    self.z,
                ])
                    as f32;
            }
        }

        self.queue.write_buffer(&self.buffer, 0, unsafe {
            std::slice::from_raw_parts(
                self.height_points.as_ptr() as *const u8,
                self.height_points.len() * size_of::<f32>(),
            )
        });
    }
    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }
}
