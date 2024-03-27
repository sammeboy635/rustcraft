
use crate::atlas_coords;
use crate::vertex::*;
use crate::atlas::*;
pub const ATLAS_SIZE: &[f32] = &[16.0,16.0];

pub const TOP: i32 = 0;
pub const BOTTOM: i32 = 1;
pub const RIGHT: i32 = 2;
pub const LEFT: i32 = 3;
pub const FRONT: i32 = 4;
pub const BACK: i32 = 5;


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
	pub fn append(&mut self, x1:f32, y1:f32, z1:f32){
		let x2 = x1+1.0;
		let y2 = y1+1.0;
		let z2 = z1+1.0;
		//front
		let faces: &[Vertex] = &[
		Vertex { position: [x1, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:FRONT },
		Vertex { position: [x2, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
		
		Vertex { position: [x1, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:FRONT },
		Vertex { position: [x2, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:FRONT },
		Vertex { position: [x1, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:FRONT },
		//back
		Vertex { position: [x2, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BACK },
		Vertex { position: [x1, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },

		Vertex { position: [x2, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BACK },
		Vertex { position: [x1, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BACK },
		Vertex { position: [x2, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BACK },
		
		//right
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
		Vertex { position: [x2, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:RIGHT },
		Vertex { position: [x2, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
		
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:RIGHT },
		Vertex { position: [x2, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:RIGHT },
		Vertex { position: [x2, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:RIGHT },
		
		// left
		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
		Vertex { position: [x1, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:LEFT },
		Vertex { position: [x1, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
		
		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:LEFT },
		Vertex { position: [x1, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:LEFT },
		Vertex { position: [x1, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:LEFT },
		
		// top
		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [x2, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },

		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
		Vertex { position: [x1, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },
		
		//5u, 4u, 1u, 5u, 1u, 0u
		// var vertices: array<vec3<f32>, 8> = array<vec3<f32>, 8>(
		// 	vec3<f32>(x1, y2, z1),   // 0
		// 	vec3<f32>(x2, y2, z1),  // 1
		// 	vec3<f32>(x2, y1, z1),   // 2
		// 	vec3<f32>(x1, y1, z1),    // 3
		// 	vec3<f32>(x2, y2, z2), // 4
		// 	vec3<f32>(x1, y2, z2),  // 5
		// 	vec3<f32>(x1, y1, z2),   // 6
		// 	vec3<f32>(x2, y1, z2)   // 7
		// );
		// Bottom
		Vertex { position: [x1, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
		Vertex { position: [x2, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:BOTTOM },
		Vertex { position: [x2, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
		
		Vertex { position: [x1, y1, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:BOTTOM },
		Vertex { position: [x2, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:BOTTOM },
		Vertex { position: [x1, y1, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:BOTTOM },
		];
		self.verts.extend(faces);
		// self.append_instance(x1, y1, z1);
	}


	pub fn append_top(&mut self, x1:f32, y1:f32, z1:f32){
		let x2 = x1+1.0;
		let y2 = y1+1.0;
		let z2 = z1+1.0;
		//front
		let faces: &[Vertex] = &[
		// top
		// var cubeIndices: array<array<u32, 6>, 6> = array<array<u32, 4>, 6>(
		// 	array<u32, 4>(0u, 1u, 2u, 3u, 0u, 0u), // FRONT
		// 	array<u32, 4>(4u, 5u, 6u, 7u, 0u, 0u), // BACK
		// 	array<u32, 4>(1u, 4u, 7u, 3u, 0u, 0u), // RIGHT
		// 	array<u32, 4>(5u, 0u, 3u, 6u, 0u, 0u), // LEFT
		// 	array<u32, 4>(5u, 4u, 1u, 4u, 1u, 6u), // TOP
		// 	array<u32, 4>(3u, 2u, 7u, 6u, 0u, 0u)  // BOTTOM
		// );
		// var vertices: array<vec3<f32>, 8> = array<vec3<f32>, 8>(
		// 	vec3<f32>(x1, y2, z1),   // 0
		// 	vec3<f32>(x2, y2, z1),  // 1
		// 	vec3<f32>(x2, y1, z1),   // 2
		// 	vec3<f32>(x1, y1, z1),    // 3
		// 	vec3<f32>(x2, y2, z2), // 4
		// 	vec3<f32>(x1, y2, z2),  // 5
		// 	vec3<f32>(x1, y1, z2),   // 6
		// 	vec3<f32>(x2, y1, z2)   // 7
		// );
		// Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		// Vertex { position: [xx, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
		// Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },

		// Vertex { position: [x, yy, zz], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		// Vertex { position: [xx, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
		// Vertex { position: [x, yy, z], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },
		
		Vertex { position: [x1, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[0], normals:TOP },
		Vertex { position: [x2, y2, z2], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[1], normals:TOP },
		Vertex { position: [x2, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[2], normals:TOP },
		Vertex { position: [x1, y2, z1], tex_coords: atlas_coords!(ATLAS[0],ATLAS[1])[3], normals:TOP },

		];
		self.verts.extend(faces);
		self.indics.extend(&[0,1,2,0,2,3]);
		// self.append_instance(x, y, z,5);
	}


	pub fn append_instance(&mut self, x:f32, y:f32, z:f32, face:i32){
		self.instance_data.push(InstanceRaw::new(x,y,z,face));
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