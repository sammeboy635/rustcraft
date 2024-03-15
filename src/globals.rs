use crate::vertex::*;

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
    
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
    
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
    
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
    
    Vertex { position: [-1.0,  1.0,  1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 1.0,  1.0,  1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [-1.0,  1.0, -1.0], tex_coords: [0.0, 1.0] },
    
    Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [-1.0, -1.0,  1.0], tex_coords: [0.0, 1.0] },
];

pub const INDICES: &[u16] = &[
    0, 1, 2,  0, 2, 3,  // Front face
    4, 5, 6,  4, 6, 7,  // Back face
    8, 9, 10,  8, 10, 11,  // Right face
    12, 13, 14,  12, 14, 15,  // Left face
    16, 17, 18,  16, 18, 19,  // Top face
    20, 21, 22,  20, 22, 23,  // Bottom face
];