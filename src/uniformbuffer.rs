pub trait UniformBuffer {
	type Uniform;
	type Buffer;
    type BindGroupLayout;
    type BindGroup;
}

pub trait Renderer {
	type RenderPipeline;
	type VertexBuffer;
	type IndexBuffer;
	type DiffuseBindGroup;
	type DiffuseTexture;
	type UniformBuffers;
	fn vertex_len(&self) -> i32;
	fn index_len(&self) -> i32;
}