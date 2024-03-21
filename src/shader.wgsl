// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

const TOP: i32 = 0;
const BOTTOM: i32 = 1;
const RIGHT: i32 = 2;
const LEFT: i32 = 3;
const FRONT: i32 = 4;
const BACK: i32 = 5;

fn get_normals(index: i32) -> vec3<f32>{
	switch(index) {
		case TOP: { return vec3<f32>(0.0,1.0,0.0);}
		case BOTTOM: { return vec3<f32>(0.0,-1.0,0.0);}
		case RIGHT: { return vec3<f32>(1.0,0.0,0.0);}
		case LEFT: { return vec3<f32>(-1.0,0.0,0.0);}
		case FRONT: { return vec3<f32>(0.0,0.0,1.0);}
		case BACK: { return vec3<f32>(0.0,0.0,-1.0);}
		default:{return vec3<f32>(0.0,0.0,0.0);}
	}
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
	let lightdir = normalize(vec3<f32>(-1.0, 1.0, -1.0));
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0); // 2.
    return out;
}
 

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

