use wgpu::util::DeviceExt;

use crate::pixel;

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub pixel_buffer: wgpu::Buffer,
    pub pixels: Vec<Vec<pixel::Pixel>>,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        resolution: (u32, u32),
    ) -> Self {
        let width = resolution.0;
        let height = resolution.1;

        let pixels: Vec<Vec<pixel::Pixel>> = {
            (0..width)
                .map(|x| {
                    (0..height)
                        .map(move |y| pixel::Pixel::new([x, y], [1.0, 1.0, 1.0]))
                        .collect()
                })
                .collect()
        };

        let pixel_data: Vec<pixel::Pixel> = pixels.clone().into_iter().flatten().collect();

        let pixel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pixel Buffer"),
            contents: bytemuck::cast_slice(&pixel_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[pixel::Vertex::desc(), pixel::Pixel::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&pixel::PIXEL_MODEL_VIEW_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&pixel::PIXEL_MODEL_VIEW_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            pixel_buffer,
            pixels,
        }
    }

    pub fn pixels_raw(&self) -> Vec<pixel::Pixel> {
        (0..self.pixels.len())
            .flat_map(|x| (0..self.pixels[x].len()).map(move |y| self.pixels[x][y]))
            .collect::<Vec<_>>()
    }
}
