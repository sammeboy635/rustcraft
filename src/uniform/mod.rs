pub mod camera;

pub use crate::uniform::camera::*;
pub trait UniformBuffer {
	type Uniform;
	type Buffer;
    type BindGroupLayout;
    type BindGroup;
}
