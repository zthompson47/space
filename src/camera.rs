use cgmath::{InnerSpace, SquareMatrix};
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, VirtualKeyCode},
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.0001;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        CameraUniform {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}

#[derive(Default)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        CameraController {
            speed,
            sensitivity,
            ..Default::default()
        }
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
            }
            VirtualKeyCode::Q => {
                self.amount_up = amount;
            }
            VirtualKeyCode::Z => {
                self.amount_down = amount;
            }
            _ => return false,
        }

        true
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: instant::Duration) {
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = cgmath::Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = cgmath::Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward =
            cgmath::Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;

        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        camera.yaw += cgmath::Rad(self.rotate_horizontal) * self.sensitivity * dt;
        camera.pitch += cgmath::Rad(-self.rotate_vertical) * self.sensitivity * dt;

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        if camera.pitch < -cgmath::Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -cgmath::Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > cgmath::Rad(SAFE_FRAC_PI_2) {
            camera.pitch = cgmath::Rad(SAFE_FRAC_PI_2);
        }
    }
}

pub struct Projection {
    aspect: f32,
    fovy: cgmath::Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<cgmath::Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Projection {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

pub struct Camera {
    pub position: cgmath::Point3<f32>,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn new<
        V: Into<cgmath::Point3<f32>>,
        Y: Into<cgmath::Rad<f32>>,
        P: Into<cgmath::Rad<f32>>,
    >(
        device: &wgpu::Device,
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        let uniform = CameraUniform::new();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
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

        Camera {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }

    fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(self.yaw.0.cos(), self.pitch.0.sin(), self.yaw.0.sin())
                .normalize(),
            cgmath::Vector3::unit_y(),
        )
    }

    pub fn update_view_proj(&mut self, projection: &Projection) {
        self.uniform.view_position = self.position.to_homogeneous().into();
        self.uniform.view_proj = (projection.calc_matrix() * self.calc_matrix()).into();
    }
}
