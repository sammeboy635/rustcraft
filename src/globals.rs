use crate::vertex::*;
pub const ATLAS_SIZE: &[f32] = &[16.0,16.0];

const TOP: i32 = 0;
const BOTTOM: i32 = 1;
const RIGHT: i32 = 2;
const LEFT: i32 = 3;
const FRONT: i32 = 4;
const BACK: i32 = 5;

const INSET_VALUE: f32 = 0.03; // Adjust this value as needed

macro_rules! atlas_coords {
    ($coord_x:expr, $coord_y:expr) => {
        [
            [
                ($coord_x as f32 + INSET_VALUE) / ATLAS_SIZE[0] as f32,
                ($coord_y as f32 + INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                (($coord_x + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[0] as f32,
                ($coord_y as f32 + INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                (($coord_x + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[0] as f32,
                (($coord_y + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
            [
                ($coord_x as f32 + INSET_VALUE) / ATLAS_SIZE[0] as f32,
                (($coord_y + 1) as f32 - INSET_VALUE) / ATLAS_SIZE[1] as f32,
            ],
        ]
    };
}
pub const ATLAS :&[u32] = & [1,15];
pub const X: &[f32] = &[0.0, 1.0];
pub const Y: &[f32] = &[0.0, 1.0];
pub const Z: &[f32] = &[0.0, 1.0];

pub const VERTICES: &[Vertex] = &[
	//front
    Vertex { position: [X[0], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
    Vertex { position: [X[1], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:FRONT },
    Vertex { position: [X[1], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
    Vertex { position: [X[0], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:FRONT },
    //back
    Vertex { position: [X[1], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
    Vertex { position: [X[0], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BACK },
    Vertex { position: [X[0], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },
    Vertex { position: [X[1], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BACK },
    //right
    Vertex { position: [X[1], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
    Vertex { position: [X[1], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:RIGHT },
    Vertex { position: [X[1], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
    Vertex { position: [X[1], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:RIGHT },
    // left
    Vertex { position: [X[0], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
    Vertex { position: [X[0], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:LEFT },
    Vertex { position: [X[0], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
    Vertex { position: [X[0], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:LEFT },
    // top
    Vertex { position: [X[0], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
    Vertex { position: [X[1], Y[1], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
    Vertex { position: [X[1], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
    Vertex { position: [X[0], Y[1], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },
    // Bottom
    Vertex { position: [X[0], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
    Vertex { position: [X[1], Y[0], Z[0]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BOTTOM },
    Vertex { position: [X[1], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
    Vertex { position: [X[0], Y[0], Z[1]], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BOTTOM },
];

pub const INDICES: &[u16] = &[
    0, 1, 2,  0, 2, 3,  // Front face
    4, 5, 6,  4, 6, 7,  // Back face
    8, 9, 10,  8, 10, 11,  // Right face
    12, 13, 14,  12, 14, 15,  // Left face
    16, 17, 18,  16, 18, 19,  // Top face
    20, 21, 22,  20, 22, 23,  // Bottom face
];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct InstanceRaw {
    model: [f32; 3],
}

#[derive(Debug, Default)]
pub struct Vertices{
	pub verts: Vec<Vertex>,
	pub indics: Vec<u16>,
	pub instance_data: Vec<InstanceRaw>,
}



impl Vertices{
	pub fn new() -> Self{
		Vertices { verts: vec![], indics: vec![], instance_data: vec![] }
	}
	pub fn append(&mut self, x:f32, y:f32, z:f32){
		let xx = x+1.0;
		let yy = y+1.0;
		let zz = z+1.0;
		//front
		let faces: &[Vertex] = &[
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:FRONT },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
		
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:FRONT },
		//back
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BACK },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },

		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BACK },
		
		//right
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:RIGHT },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
		
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:RIGHT },
		
		// left
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:LEFT },
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
		
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:LEFT },
		
		// top
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },

		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },
		
		// Bottom
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BOTTOM },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
		
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BOTTOM },
		];
		self.verts.extend(faces);
		self.indics.extend(INDICES);
	}
	pub fn append_vert(&mut self, x:f32, y:f32, z:f32){
		let xx = x+1.0;
		let yy = y+1.0;
		let zz = z+1.0;
		//front
		let faces: &[Vertex] = &[
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:FRONT },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:FRONT },
		//back
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BACK },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BACK },
		//right
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:RIGHT },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:RIGHT },
		// left
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:LEFT },
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:LEFT },
		// top
		Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
		Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
		Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },
		// Bottom
		Vertex { position: [x, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
		Vertex { position: [xx, y, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BOTTOM },
		Vertex { position: [xx, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
		Vertex { position: [x, y, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BOTTOM },
		];
		self.verts.extend(faces);
		self.indics.extend(INDICES);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_atlas_macro() {
		let mut vertices = Vertices::default();
		println!("{:?}", vertices);
		vertices.append(0.0, 0.0, 0.0);
		vertices.append(10.0, 0.0, 10.0);
		println!("{:?}", vertices);
        println!("{:?}", atlas_coords!(15,8));
		println!("{:?}", X[0])
    }
}