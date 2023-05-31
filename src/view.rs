use std::collections::VecDeque;

use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    camera::{Camera, CameraController, Projection},
    draw_shape::{DrawShape, DrawShapePipeline},
    resources::load_texture,
    rotation::RotationY,
    skybox::Skybox,
    texture::Texture,
};

pub struct GuiState {
    slider: f32,
}

#[derive(Default)]
pub struct Keys {
    pub rotation: bool,
}

pub struct RenderView {
    size: winit::dpi::PhysicalSize<u32>,
    #[allow(unused)]
    scale_factor: f32,
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
    draw_shapes: VecDeque<DrawShapePipeline>,
    pub keys: Keys,
    skybox: Skybox,
    skybox_pipeline: wgpu::RenderPipeline,
    pub egui_context: egui::Context,
    egui_renderer: egui_wgpu::Renderer,
    pub egui_repaint: bool,
    texture: Texture,
    gui: GuiState,
    //noise: Vec<f32>,
}

impl RenderView {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..wgpu::InstanceDescriptor::default()
        });

        // SAFETY: `View` is created in the main thread and `window` remains valid
        // for the lifetime of `surface`.
        let surface = unsafe { instance.create_surface(window).unwrap() };

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
        let capabilities = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![capabilities.formats[0]],
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

        let texture = load_texture("baba.png", &device, &queue).await.unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &rotation.bind_group_layout,
                &camera.bind_group_layout,
                &skybox.bind_group_layout,
                &texture.bind_group_layout,
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
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                //targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let egui_context = egui::Context::default();
        let egui_renderer = egui_wgpu::Renderer::new(&device, config.format, None, 1);

        //let noise = simdnoise::NoiseBuilder::fbm_1d(256).generate_scaled(0.0, 1.0);

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
            egui_context,
            egui_renderer,
            egui_repaint: false,
            texture,
            scale_factor,
            gui: GuiState { slider: 1.0 },
            //noise,
        }
    }

    pub fn push_shape(&mut self, shape: DrawShape) {
        self.draw_shapes.push_back(DrawShapePipeline::new(
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
            self.rotation.increment_angle(&self.queue, step);
        }
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera.update_view_proj(&self.projection);
        self.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );

        use easer::functions::{Easing, Sine};
        let slider = Sine::ease_in_out(self.gui.slider, 0.0, 1.0, 1.0);
        self.texture.update(&self.queue, slider);
    }

    pub fn render(&mut self, egui_input: egui::RawInput) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_bind_group(3, &self.texture.bind_group, &[]);
            render_pass.set_pipeline(&self.skybox_pipeline);
            render_pass.draw(0..3, 0..1);

            drop(render_pass);
            for shape in self.draw_shapes.iter_mut() {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load, // Clear(wgpu::Color::default()),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_bind_group(0, &self.rotation.bind_group, &[]);
                render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
                render_pass.set_bind_group(2, &self.skybox.bind_group, &[]);
                render_pass.set_bind_group(3, &self.texture.bind_group, &[]);
                render_pass.set_pipeline(&shape.pipeline);
                render_pass.draw(0..shape.shape.vertex_count, 0..1);
            }
        }

        /*
        for shape in self.draw_shapes.iter_mut() {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Clear(wgpu::Color::default()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_bind_group(0, &self.rotation.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera.bind_group, &[]);
            render_pass.set_bind_group(2, &self.skybox.bind_group, &[]);
            render_pass.set_bind_group(3, &self.texture.bind_group, &[]);
            render_pass.set_pipeline(&shape.pipeline);
            render_pass.draw(0..shape.shape.vertex_count, 0..1);
        }
        */

        // Egui
        let full_output = self.egui_context.run(egui_input, |ctx| {
            egui::Area::new("space_gui")
                //.fixed_pos(egui::pos2(10., 10.))
                .show(ctx, |ui| {
                    ui.label("Hello egui!");
                    ui.add(egui::Slider::new(&mut self.gui.slider, 0.0..=1.0).text("Slider"));
                });
        });

        let clipped_primitives: Vec<egui::epaint::ClippedPrimitive> =
            self.egui_context.tessellate(full_output.shapes);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }
        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        let screen_descriptor = egui_wgpu::renderer::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: 2.0, //self.scale_factor,
        };

        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            clipped_primitives.as_slice(),
            &screen_descriptor,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.egui_renderer.render(
                &mut render_pass,
                clipped_primitives.as_slice(),
                &screen_descriptor,
            );
        }

        // Send queue to GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
