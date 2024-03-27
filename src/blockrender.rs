use crate::texture::*;
use crate::uniform::*;
use crate::vertex::*;
use crate::globals::*;
use crate::atlas::*;

use wgpu::util::DeviceExt;



pub struct BlockRender {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
	pub instance_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub diffuse_texture: Texture,
    pub camera_wgpu: CameraUniformBuffer,
	pub vertices: Vertices,
}

impl BlockRender {
	pub fn new(device: & wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration, camera_wgpu : CameraUniformBuffer) -> Self{
		// TEXTURE START
		
		let mut vertices = Vertices::default();
		// vertices.append(0.0, 2.0, 0.0);
		vertices.append(0.0, 0.0, 0.0);
		for x in 0..100{
			for y in 0..100{
				vertices.append_instance(x as f32,y as f32,0.0);
			}
		}

		let diffuse_bytes = include_bytes!("texture.png"); 
		let diffuse_texture = Texture::from_bytes(&device, &queue, diffuse_bytes, "texture.png").unwrap(); 

		let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

		let diffuse_bind_group = device.create_bind_group(
			&wgpu::BindGroupDescriptor {
				layout: &texture_bind_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
					}
				],
				label: Some("diffuse_bind_group"),
			}
		);
		
		// TEXTURE END
		
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
		});

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[
				&texture_bind_group_layout,
				&camera_wgpu.bind_group_layout,
			],
			push_constant_ranges: &[],
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main", // 1.
				buffers: &[Vertex::desc(),InstanceRaw::desc()],
			},
			fragment: Some(wgpu::FragmentState { // 3.
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState { // 4.
					format: config.format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList, // 1.
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw, // 2.
				cull_mode: Some(wgpu::Face::Back),
				// Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
				polygon_mode: wgpu::PolygonMode::Fill,
				// Requires Features::DEPTH_CLIP_CONTROL
				unclipped_depth: false,
				// Requires Features::CONSERVATIVE_RASTERIZATION
				conservative: false,
			},
			depth_stencil: Some(wgpu::DepthStencilState {
				format: Texture::DEPTH_FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less, // 1.
				stencil: wgpu::StencilState::default(), // 2.
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState {
				count: 1, // 2.
				mask: !0, // 3.
				alpha_to_coverage_enabled: false, // 4.
			},
			multiview: None, // 5.
		});

		let vertex_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Vertex Buffer"),
				contents: bytemuck::cast_slice(&vertices.verts),
				usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
			}
		);

		let instance_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Instance Buffer"),
				contents: bytemuck::cast_slice(&vertices.instance_data),
				usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
			}
		);

		let index_buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Index Buffer"),
				contents: bytemuck::cast_slice(&vertices.indics),
				usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
			}
		);
		// let num_indices = INDICES.len() as u32;
		
		Self{
			render_pipeline,
			vertex_buffer,
			index_buffer,
			instance_buffer,
			diffuse_bind_group,
			diffuse_texture,
			camera_wgpu,
			vertices,
		}
	}
	// pub fn render_pass(&Self) {

	// }
}