use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RotationRaw {
    transform: [[f32; 4]; 4],
}

pub struct RotationY {
    raw: RotationRaw,
    angle: cgmath::Rad<f32>,
    buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl RotationY {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = RotationRaw {
            transform: cgmath::Matrix4::identity().into(),
        };
        let angle = cgmath::Rad(0.0);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[raw]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        RotationY {
            raw,
            angle,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn increment_angle(&mut self, queue: &wgpu::Queue, incr: cgmath::Rad<f32>) {
        self.angle += incr;
        self.raw.transform = cgmath::Matrix4::from_angle_y(self.angle).into();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }
}
