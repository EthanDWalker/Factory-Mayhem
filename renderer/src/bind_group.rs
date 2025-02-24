use wgpu::util::DeviceExt;

pub struct BindGroup {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
}

impl BindGroup {
    pub fn new(
        device: &wgpu::Device,
        buffer_layout: &wgpu::util::BufferInitDescriptor,
        bind_group_layout: &wgpu::BindGroupLayoutDescriptor,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer_init(buffer_layout);
        let layout = device.create_bind_group_layout(bind_group_layout);

        let bind_group = {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label,
            })
        };

        Self {
            buffer,
            bind_group,
            layout,
        }
    }
}
