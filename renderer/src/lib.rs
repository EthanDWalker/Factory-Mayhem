mod bind_group;
mod pipeline;
mod pixel;
mod render;

pub mod factory_belt;
pub mod circle;
pub mod sprite;
pub mod sprite_list;
pub mod player;
pub mod factory_crate;

use std::sync::Arc;

const RESOLUTION_HEIGHT: u32 = 200;

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub resolution: (u32, u32),
    resolution_bind_group: bind_group::BindGroup,
    pipeline: pipeline::Pipeline,
    pub last_render_time: std::time::Instant,
}

impl<'a> Renderer<'a> {
    pub async fn new(
        window: Arc<winit::window::Window>,
    ) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
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

        surface.configure(&device, &config);

        let aspect_ratio: f32 = (size.width as f32) / (size.height as f32);
        let res_width: u32 = (RESOLUTION_HEIGHT as f32 * aspect_ratio) as u32;

        let resolution_bind_group = bind_group::BindGroup::new(
            &device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Resolution Buffer"),
                contents: bytemuck::cast_slice(&[[res_width, RESOLUTION_HEIGHT]]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            },
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("resolution_bind_group_layout"),
            },
            Some("Resolution Bind_Group"),
        );

        let pipeline = pipeline::Pipeline::new(
            &device,
            config.format,
            &[&resolution_bind_group.layout],
            (res_width, RESOLUTION_HEIGHT),
        );

        Self {
            surface,
            device,
            queue,
            config,
            resolution: (res_width, RESOLUTION_HEIGHT),
            resolution_bind_group,
            pipeline,
            last_render_time: std::time::Instant::now(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            let aspect_ratio: f32 = (new_size.width as f32) / (new_size.height as f32);
            println!("{}", aspect_ratio);
            self.resolution = (
                (RESOLUTION_HEIGHT as f32 * aspect_ratio) as u32,
                RESOLUTION_HEIGHT,
            );
            self.surface.configure(&self.device, &self.config);
            self.pipeline = pipeline::Pipeline::new(
                &self.device,
                self.config.format,
                &[&self.resolution_bind_group.layout],
                self.resolution,
            );

            self.queue.write_buffer(
                &self.resolution_bind_group.buffer,
                0,
                bytemuck::cast_slice(&[[self.resolution.0, self.resolution.1]]),
            );
        }
    }

    pub fn update(&mut self, input: &mut systems::Input, sprites: &mut sprite_list::SpriteList, reset_level: &mut bool, win_level: &mut bool, level: u32, delta_time: f32) {
        render::update(self, input, sprites, reset_level, win_level, level, delta_time);
    }

    pub fn render(&mut self) {
        render::render(self).expect("Failed to render");
    }
}
