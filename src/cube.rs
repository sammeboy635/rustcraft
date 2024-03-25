use crate::globals::*;
use crate::vertex::*;

struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}





#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_my_function() {
        // let mut vertices = Vec::new();
		// let mut indices = Vec::new();

		// create_block_vertices(0b00010001, &mut vertices, &mut indices, [0.0, 0.0, 0.0]);
		// create_block_vertices(0b00010001, &mut vertices, &mut indices, [1.0, 0.0, 0.0]);
		// create_block_vertices(0b00010001, &mut vertices, &mut indices, [0.0, 1.0, 0.0]);
		// println!("{:?}", vertices);
		// println!("{:?}", indices);
		
		let mut verts = VERTICES;
		let mut vertice = vec![Vertex{position:[0.0,0.0,0.0], tex_coords:[0.0,1.0], normals:0}];
		
		println!("{:?}", verts.len());
		let mut terts = VERTICES;
		println!("{:?}",[verts, terts].concat().len());
		// verts(terts)
    }
}