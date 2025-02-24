#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [u32; 2],
}

pub const PIXEL_MODEL_VIEW_VERTICES: [Vertex; 4] = [
    Vertex { position: [1, 0] },
    Vertex { position: [0, 0] },
    Vertex { position: [0, 1] },
    Vertex { position: [1, 1] },
];

pub const PIXEL_MODEL_VIEW_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Uint32x2,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Pixel {
    pub position: [u32; 2],
    pub color: [f32; 3],
    pub lighting: f32,
}

impl Pixel {
    pub fn new(pos: [u32; 2], rgb_color: [f32; 3]) -> Self {
        let position = pos;
        let color = rgb_color;
        let lighting = 1.0;

        Self { position, color, lighting }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Pixel>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[i32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[i32; 5]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}
