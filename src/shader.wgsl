struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normals: i32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normals: vec3<f32>,
};

const TOP: i32 = 0;
const BOTTOM: i32 = 1;
const RIGHT: i32 = 2;
const LEFT: i32 = 3;
const FRONT: i32 = 4;
const BACK: i32 = 5;

fn get_normals(index: i32) -> vec3<f32> {
    switch(index) {
        case TOP: {return vec3<f32>(0.0, 1.0, 0.0);}
        case BOTTOM: {return vec3<f32>(0.0, -1.0, 0.0);}
        case RIGHT: {return vec3<f32>(1.0, 0.0, 0.0);}
        case LEFT: {return vec3<f32>(-1.0, 0.0, 0.0);}
        case FRONT: {return vec3<f32>(0.0, 0.0, 1.0);}
        case BACK: {return vec3<f32>(0.0, 0.0, -1.0);}
        default: {return vec3<f32>(0.0, 0.0, 0.0);}
    }
}

fn calculateTangent(normal: vec3<f32>) -> vec3<f32> {
    // Arbitrarily select a direction that's not parallel to the normal
    var tangent = vec3<f32>(1.0, 0.0, 0.0);
    
    // If the normal is almost parallel to the tangent, select a different direction
    if (abs(dot(normal, tangent)) > 0.9) {
        tangent = vec3<f32>(0.0, 1.0, 0.0);
    }
        
    // Calculate tangent as the cross product of the normal and the chosen tangent
    tangent = normalize(cross(normal, tangent));
    
    return tangent;
}

fn calculateBitangent(normal: vec3<f32>, tangent: vec3<f32>) -> vec3<f32> {
    // Calculate bitangent as the cross product of the normal and tangent
    var bitangent = normalize(cross(normal, tangent));
    
    return bitangent;
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var normals: vec3<f32> = get_normals(model.normals);
    let lightdir = normalize(vec3<f32>(-1.0, 1.0, -1.0));
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.normals = normals;
    return out;
}
 
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample textures
    let diffuseColor: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    // Use provided normals
    let normal: vec3<f32> = in.normals;
    
    // Calculate tangent and bitangent
    let tangent: vec3<f32> = calculateTangent(normal);
    let bitangent: vec3<f32> = calculateBitangent(normal, tangent);
    
    // Transform normal to view space (if needed)
    let viewNormal: vec3<f32> = normalize(normal); // Assuming normal is already in view space
    
    // Calculate lighting (Lambertian diffuse lighting)
    let lightDirection: vec3<f32> = normalize(vec3<f32>(-1.0, 1.0, -1.0));
    let diffuseIntensity: f32 = max(0.0,dot(viewNormal, lightDirection));
    
    // Adjust the diffuse color to make the non-light side slightly darker
    let darknessFactor: f32 = 0.6; // You can adjust this value as needed
    
    // Calculate the minimum brightness based on diffuseIntensity
    let minBrightness: f32 = 0.2; // Adjust this value as needed, it represents the minimum brightness
    
    // Apply minimum brightness to the diffuse color based on its intensity
    let finalColor: vec4<f32> = diffuseColor * (diffuseIntensity * darknessFactor) + minBrightness;

    return finalColor;
}

