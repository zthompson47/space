use wgpu::util::DeviceExt;

pub struct Skybox {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl Skybox {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let size = wgpu::Extent3d {
            //width: 2048,
            //height: 2048,
            width: 200,
            height: 200,
            depth_or_array_layers: 6,
        };
        /*
        let layer_size = wgpu::Extent3d {
            depth_or_array_layers: 1,
            ..size
        };
        let max_mips = layer_size.max_mips(wgpu::TextureDimension::D2);
        */

        fn pack_skybox(accum: &mut Vec<u8>, image_bytes: &[u8]) {
            let decoded = image::load_from_memory(image_bytes).unwrap().into_rgba8();
            accum.extend_from_slice(decoded.as_raw());
        }
        let mut data = Vec::new();
        //pack_skybox(&mut data, include_bytes!("../res/skybox/right.jpg"));
        //pack_skybox(&mut data, include_bytes!("../res/skybox/left.jpg"));
        //pack_skybox(&mut data, include_bytes!("../res/skybox/top.jpg"));
        //pack_skybox(&mut data, include_bytes!("../res/skybox/bottom.jpg"));
        //pack_skybox(&mut data, include_bytes!("../res/skybox/front.jpg"));
        //pack_skybox(&mut data, include_bytes!("../res/skybox/back.jpg"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_right.png"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_left.png"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_top.png"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_bottom.png"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_front.png"));
        pack_skybox(&mut data, include_bytes!("../res/cmb/cmb_back.png"));

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
            },
            &data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..wgpu::TextureViewDescriptor::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Skybox {
            bind_group_layout,
            bind_group,
        }
    }
}
