use std::collections::VecDeque;

use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    camera::{Camera, CameraController, Projection},
    draw_shape::{DrawShape, DrawShapeRenderPass},
    rotation::RotationY,
    skybox::Skybox,
};

#[derive(Default)]
pub struct Keys {
    pub rotation: bool,
}

pub struct RenderView {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    camera: Camera,
    shader: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    rotation: RotationY,
    pub camera_controller: CameraController,
    projection: Projection,
    pub mouse_pressed: bool,
    draw_shapes: VecDeque<DrawShapeRenderPass>,
    pub keys: Keys,
    skybox: Skybox,
    skybox_pipeline: wgpu::RenderPipeline,
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
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let rotation = RotationY::new(&device);

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

        let skybox = Skybox::new(&device, &queue);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &rotation.bind_group_layout,
                &camera.bind_group_layout,
                &skybox.bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let skybox_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sky"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_sky",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_sky",
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
            /*depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),*/
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        RenderView {
            size,
            surface,
            device,
            queue,
            config,
            keys,
            camera,
            camera_controller,
            projection,
            mouse_pressed: false,
            draw_shapes: VecDeque::new(),
            shader,
            pipeline_layout,
            rotation,
            skybox,
            skybox_pipeline,
        }
    }

    pub fn push_shape(&mut self, shape: DrawShape) {
        self.draw_shapes.push_back(DrawShapeRenderPass::new(
            &self.device,
            &self.config,
            shape,
            &self.shader,
            &self.pipeline_layout,
        ));
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
        let step = dt.as_secs_f32();

        if self.keys.rotation {
            self.rotation
                .increment_angle(&self.queue, cgmath::Rad(step));
        }

        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera.update_view_proj(&self.projection);

        self.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::default()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_bind_group(0, &self.rotation.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.set_bind_group(2, &self.skybox.bind_group, &[]);

            render_pass.set_pipeline(&self.skybox_pipeline);
            render_pass.draw(0..3, 0..1);

            for shape in self.draw_shapes.iter_mut() {
                render_pass.set_pipeline(&shape.pipeline);
                render_pass.draw(0..shape.shape.vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
