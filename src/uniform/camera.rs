use wgpu::util::DeviceExt;
use cgmath::*;


use crate::player::*;
use crate::uniform::*;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraUniformBuffer {
    pub uniform: CameraUniform,
	pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformBuffer for CameraUniformBuffer {
	type Uniform = CameraUniform;
	type Buffer = wgpu::Buffer;
    type BindGroupLayout = wgpu::BindGroupLayout;
    type BindGroup = wgpu::BindGroup;
}

impl CameraUniformBuffer {
    pub fn new(device: &wgpu::Device, uniform: CameraUniform) -> Self {

		let buffer = device.create_buffer_init(
			&wgpu::util::BufferInitDescriptor {
				label: Some("Camera Buffer"),
				contents: bytemuck::cast_slice(&[uniform]),
				usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			}
		);

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("Camera Bind Group Layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}
			],
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}

