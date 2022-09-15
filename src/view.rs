use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    camera::{Camera, CameraController, Projection},
    draw_shape::{DrawShape, DrawShapeRenderPass},
};

#[derive(Default)]
pub struct Keys {
    pub rotation: bool,
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub mouse_pressed: bool,
    pub rotate_horizontal: f32,
    pub rotate_vertical: f32,
    pub scroll: f32,
}

pub struct RenderView {
    size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    camera: Camera,
    pub camera_controller: CameraController,
    projection: Projection,
    pub mouse_pressed: bool,
    pyramid3_render_pass: DrawShapeRenderPass,
    pyramid4_render_pass: DrawShapeRenderPass,
    pub keys: Keys,
}

impl RenderView {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // SAFETY: `View` is created in the main thread and `window` remains valid
        // for the lifetime of `surface`.
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    //features: wgpu::Features::empty(),
                    features: wgpu::Features::PUSH_CONSTANTS,

                    #[cfg(target_arch = "wasm32")]
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),

                    #[cfg(not(target_arch = "wasm32"))]
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let keys = Keys::default();

        let projection =
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_controller = CameraController::new(4.0, 0.4);
        let mut camera = Camera::new(
            &device,
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
        );
        camera.update_view_proj(&projection);

        // Shapes.
        let pyramid = DrawShape {
            vertex_fn: "vs_pyramid",
            vertex_count: 9,
        };
        let triangle_render_pass =
            DrawShapeRenderPass::new(&device, &config, pyramid, &camera.bind_group_layout);
        let pyramid4 = DrawShape {
            vertex_fn: "vs_pyramid4",
            vertex_count: 12,
        };
        let shape_render_pass =
            DrawShapeRenderPass::new(&device, &config, pyramid4, &camera.bind_group_layout);

        RenderView {
            size,
            surface,
            device,
            queue,
            config,
            pyramid3_render_pass: triangle_render_pass,
            keys,
            pyramid4_render_pass: shape_render_pass,
            camera,
            camera_controller,
            projection,
            mouse_pressed: false,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
        self.projection.resize(new_size.width, new_size.height);
    }

    pub fn recover(&mut self) {
        self.resize(self.size);
    }

    pub fn update(&mut self, dt: instant::Duration) {
        self.pyramid3_render_pass
            .update(&self.queue, dt, &mut self.keys);
        self.pyramid4_render_pass
            .update(&self.queue, dt, &mut self.keys);

        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera.update_view_proj(&self.projection);

        self.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.pyramid3_render_pass
            .render(&mut encoder, &view, &self.camera.bind_group);
        self.pyramid4_render_pass
            .render(&mut encoder, &view, &self.camera.bind_group);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
