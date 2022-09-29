#[derive(Debug)]
pub struct DrawShape {
    pub vertex_fn: &'static str,
    pub fragment_fn: &'static str,
    pub vertex_count: u32,
}

#[derive(Debug)]
pub struct DrawShapePipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub shape: DrawShape,
}

impl DrawShapePipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shape: DrawShape,
        shader: &wgpu::ShaderModule,
        pipeline_layout: &wgpu::PipelineLayout,
    ) -> Self {
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
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
                module: shader,
                entry_point: shape.fragment_fn,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        DrawShapePipeline { pipeline, shape }
    }
}
