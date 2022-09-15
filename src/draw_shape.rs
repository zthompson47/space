use wgpu::include_wgsl;

use crate::{rotation::RotationY, view::Keys};

pub struct DrawShape {
    pub vertex_fn: &'static str,
    pub vertex_count: u32,
}

pub struct DrawShapeRenderPass {
    pipeline: wgpu::RenderPipeline,
    rotation: RotationY,
    shape: DrawShape,
}

impl DrawShapeRenderPass {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shape: DrawShape,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let rotation = RotationY::new(device);
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&rotation.bind_group_layout, camera_bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: shape.vertex_fn,
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Requires `Features::DEPTH_CLIP_CONTROL`.
                unclipped_depth: false,
                // Anything but Fill requires `Features::NON_FILL_POLYGON_MODE`.
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires `Features::CONSERVATIVE_RASTERIZATION`.
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        DrawShapeRenderPass {
            pipeline,
            rotation,
            shape,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, dt: instant::Duration, keys: &mut Keys) {
        let step = dt.as_millis() as f32 * 0.0006;
        if keys.rotation {
            self.rotation.increment_angle(queue, cgmath::Rad(step));
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        camera_bind_group: &wgpu::BindGroup,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    //load: wgpu::LoadOp::Clear(wgpu::Color::default()),
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.rotation.bind_group, &[]);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.draw(0..self.shape.vertex_count, 0..1);
    }
}
