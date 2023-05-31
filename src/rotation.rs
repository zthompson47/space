use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

use noize::{Ease, PNoise1};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RotationRaw {
    transform: [[f32; 4]; 4],
    jitter: [f32; 4],
}

pub struct RotationY {
    raw: RotationRaw,
    angle: cgmath::Rad<f32>,
    buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    noise: PNoise1,
    jitter: RgbaNoise,
}

struct RgbaNoise {
    r: PNoise1,
    g: PNoise1,
    b: PNoise1,
    a: PNoise1,
}

impl Default for RgbaNoise {
    fn default() -> Self {
        Self {
            r: PNoise1::new(47, 64, 1024, Ease::SmoothStep),
            g: PNoise1::new(48, 64, 1024, Ease::SmoothStep),
            b: PNoise1::new(49, 64, 1024, Ease::SmoothStep),
            a: PNoise1::new(50, 64, 1024, Ease::SmoothStep),
        }
    }
}

impl Iterator for RgbaNoise {
    type Item = [f32; 4];

    fn next(&mut self) -> Option<Self::Item> {
        Some([
            self.r.next().unwrap(),
            self.g.next().unwrap(),
            self.b.next().unwrap(),
            self.a.next().unwrap(),
        ])
    }
}

impl RotationY {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut jitter = RgbaNoise::default();

        let raw = RotationRaw {
            transform: cgmath::Matrix4::identity().into(),
            jitter: jitter.next().unwrap(),
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

        let noise = PNoise1::new(47, 8, 1024, Ease::Back);

        RotationY {
            raw,
            angle,
            buffer,
            bind_group_layout,
            bind_group,
            noise,
            jitter,
        }
    }

    pub fn increment_angle(&mut self, queue: &wgpu::Queue, dt: f32) {
        let incr = cgmath::Rad(self.noise.next().unwrap() * dt);
        self.angle += incr;
        self.raw.transform = cgmath::Matrix4::from_angle_y(self.angle).into();
        self.raw.jitter = self.jitter.next().unwrap();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.raw]));
    }
}
