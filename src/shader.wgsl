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

struct InstanceInput {
    @location(5) position: vec3<f32>,
    @location(6) face: i32,
}

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




fn generate_face_vertices(pos: vec3<f32>, face_id: i32, vertex_index: u32) -> vec4<f32> {
    let x1: f32 = pos.x;
    let x2: f32 = x1 + 1.0;
    let y1: f32 = pos.y;
    let y2: f32 = y1 + 1.0;
    let z1: f32 = pos.z;
    let z2: f32 = z1 + 1.0;
    
    var cubeIndices: array<array<u32, 4>, 6> = array<array<u32, 4>, 6>(
        array<u32, 4>(5u, 4u, 1u, 0u), // TOP
        array<u32, 4>(3u, 2u, 7u, 6u), // BOTTOM
        array<u32, 4>(1u, 4u, 7u, 3u), // RIGHT
        array<u32, 4>(5u, 0u, 3u, 6u), // LEFT
        array<u32, 4>(0u, 1u, 2u, 3u), // FRONT
        array<u32, 4>(4u, 5u, 6u, 7u)  // BACK
    );
    
    var vertices: array<vec3<f32>, 8> = array<vec3<f32>, 8>(
        vec3<f32>(x1, y2, z1),   // 0
        vec3<f32>(x2, y2, z1),   // 1
        vec3<f32>(x2, y1, z1),   // 2
        vec3<f32>(x1, y1, z1),   // 3
        vec3<f32>(x2, y2, z2),   // 4
        vec3<f32>(x1, y2, z2),   // 5
        vec3<f32>(x1, y1, z2),   // 6
        vec3<f32>(x2, y1, z2)    // 7
    );

    return vec4<f32>(vertices[cubeIndices[face_id][vertex_index]], 1.0);
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput, 
    @builtin(vertex_index) index: u32,
    @builtin(instance_index) inst_index: u32
    ) -> VertexOutput {
    // if (inst_index == 0u){
    //     discard;
    // }
    var normals: vec3<f32> = get_normals(model.normals);
    let lightdir = normalize(vec3<f32>(-1.0, 1.0, -1.0));
    let new_model: vec4<f32> = generate_face_vertices(instance.position, instance.face, index);
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * new_model;
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

