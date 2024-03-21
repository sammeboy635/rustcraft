#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug)]
struct BlockTexture {
    left: [[f32; 2]; 6],
    right: [[f32; 2]; 6],
    up: [[f32; 2]; 6],
    down: [[f32; 2]; 6],
    front: [[f32; 2]; 6],
    back: [[f32; 2]; 6],
}

fn generate_texture_coordinates(
    atlas_size: (u32, u32),
    inset: (f32, f32),
    texture_id: u32,
) -> [[f32; 2]; 6] {
    let (atlas_width, atlas_height) = atlas_size;
    let (inset_x, inset_y) = inset;

    let tile_width = 1.0 / atlas_width as f32;
    let tile_height = 1.0 / atlas_height as f32;

    let (tile_x, tile_y) = (
        (texture_id % atlas_width) as f32,
        (texture_id / atlas_width) as f32,
    );

    let left = tile_x * tile_width + inset_x;
    let right = (tile_x + 1.0) * tile_width - inset_x;
    let top = tile_y * tile_height + inset_y;
    let bottom = (tile_y + 1.0) * tile_height - inset_y;

    [
        [left, top],
        [right, top],
        [right, bottom],
        [right, bottom],
        [left, bottom],
        [left, top],
    ]
}

fn generate_block_texture(
    atlas_size: (u32, u32),
    inset: (f32, f32),
    texture_ids: [u32; 6],
) -> BlockTexture {
    BlockTexture {
        left: generate_texture_coordinates(atlas_size, inset, texture_ids[0]),
        right: generate_texture_coordinates(atlas_size, inset, texture_ids[1]),
        up: generate_texture_coordinates(atlas_size, inset, texture_ids[2]),
        down: generate_texture_coordinates(atlas_size, inset, texture_ids[3]),
        front: generate_texture_coordinates(atlas_size, inset, texture_ids[4]),
        back: generate_texture_coordinates(atlas_size, inset, texture_ids[5]),
    }
}

fn make_cube_data(mut vertices: Vec<f32>, show: [bool; 6], block: Vec3, tex: &BlockTexture) -> Vec<f32> {
    let l = &tex.left;
    let r = &tex.right;
    let u = &tex.up;
    let d = &tex.down;
    let f = &tex.front;
    let b = &tex.back;

    let x = block.x as f32;
    let y = block.y as f32;
    let z = block.z as f32;

    if show[0] {
        // left
        vertices.extend_from_slice(&[
            x - 0.5, y - 0.5, z - 0.5, l[0][0], l[0][1], -1.0, 0.0, 0.0,
            x - 0.5, y - 0.5, z + 0.5, l[1][0], l[1][1], -1.0, 0.0, 0.0,
            x - 0.5, y + 0.5, z + 0.5, l[2][0], l[2][1], -1.0, 0.0, 0.0,
            x - 0.5, y + 0.5, z + 0.5, l[3][0], l[3][1], -1.0, 0.0, 0.0,
            x - 0.5, y + 0.5, z - 0.5, l[4][0], l[4][1], -1.0, 0.0, 0.0,
            x - 0.5, y - 0.5, z - 0.5, l[5][0], l[5][1], -1.0, 0.0, 0.0,
        ]);
    }

    if show[1] {
        // right
        vertices.extend_from_slice(&[
            x + 0.5, y - 0.5, z + 0.5, r[0][0], r[0][1], 1.0, 0.0, 0.0,
            x + 0.5, y - 0.5, z - 0.5, r[1][0], r[1][1], 1.0, 0.0, 0.0,
            x + 0.5, y + 0.5, z - 0.5, r[2][0], r[2][1], 1.0, 0.0, 0.0,
            x + 0.5, y + 0.5, z - 0.5, r[3][0], r[3][1], 1.0, 0.0, 0.0,
            x + 0.5, y + 0.5, z + 0.5, r[4][0], r[4][1], 1.0, 0.0, 0.0,
            x + 0.5, y - 0.5, z + 0.5, r[5][0], r[5][1], 1.0, 0.0, 0.0,
        ]);
    }

    if show[2] {
        // top
        vertices.extend_from_slice(&[
            x - 0.5, y + 0.5, z + 0.5, u[0][0], u[0][1], 0.0, 1.0, 0.0,
            x + 0.5, y + 0.5, z + 0.5, u[1][0], u[1][1], 0.0, 1.0, 0.0,
            x + 0.5, y + 0.5, z - 0.5, u[2][0], u[2][1], 0.0, 1.0, 0.0,
            x + 0.5, y + 0.5, z - 0.5, u[3][0], u[3][1], 0.0, 1.0, 0.0,
            x - 0.5, y + 0.5, z - 0.5, u[4][0], u[4][1], 0.0, 1.0, 0.0,
            x - 0.5, y + 0.5, z + 0.5, u[5][0], u[5][1], 0.0, 1.0, 0.0,
        ]);
    }

    if show[3] {
        // bottom
        vertices.extend_from_slice(&[
            x - 0.5, y - 0.5, z - 0.5, d[0][0], d[0][1], 0.0, -1.0, 0.0,
            x + 0.5, y - 0.5, z - 0.5, d[1][0], d[1][1], 0.0, -1.0, 0.0,
            x + 0.5, y - 0.5, z + 0.5, d[2][0], d[2][1], 0.0, -1.0, 0.0,
            x + 0.5, y - 0.5, z + 0.5, d[3][0], d[3][1], 0.0, -1.0, 0.0,
            x - 0.5, y - 0.5, z + 0.5, d[4][0], d[4][1], 0.0, -1.0, 0.0,
            x - 0.5, y - 0.5, z - 0.5, d[5][0], d[5][1], 0.0, -1.0, 0.0,
        ]);
    }

    if show[4] {
        // front
        vertices.extend_from_slice(&[
            x - 0.5, y - 0.5, z + 0.5, f[0][0], f[0][1], 0.0, 0.0, 1.0,
            x + 0.5, y - 0.5, z + 0.5, f[1][0], f[1][1], 0.0, 0.0, 1.0,
            x + 0.5, y + 0.5, z + 0.5, f[2][0], f[2][1], 0.0, 0.0, 1.0,
            x + 0.5, y + 0.5, z + 0.5, f[3][0], f[3][1], 0.0, 0.0, 1.0,
            x - 0.5, y + 0.5, z + 0.5, f[4][0], f[4][1], 0.0, 0.0, 1.0,
            x - 0.5, y - 0.5, z + 0.5, f[5][0], f[5][1], 0.0, 0.0, 1.0,
        ]);
    }

    if show[5] {
        // back
        vertices.extend_from_slice(&[
            x + 0.5, y - 0.5, z - 0.5, b[0][0], b[0][1], 0.0, 0.0, -1.0,
            x - 0.5, y - 0.5, z - 0.5, b[1][0], b[1][1], 0.0, 0.0, -1.0,
            x - 0.5, y + 0.5, z - 0.5, b[2][0], b[2][1], 0.0, 0.0, -1.0,
            x - 0.5, y + 0.5, z - 0.5, b[3][0], b[3][1], 0.0, 0.0, -1.0,
            x + 0.5, y + 0.5, z - 0.5, b[4][0], b[4][1], 0.0, 0.0, -1.0,
            x + 0.5, y - 0.5, z - 0.5, b[5][0], b[5][1], 0.0, 0.0, -1.0,
        ]);
    }

    vertices
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_atlas() {
        let atlas_size = (16, 16);
		let inset = (0.01, 0.01);
		let texture_ids = [164, 165, 166, 167, 168, 169]; // Specify texture IDs for each face

		let block_texture = generate_block_texture(atlas_size, inset, texture_ids);
		println!("{:?}",block_texture);
		let show = [true, true, true, true, true, true];
		let block = Vec3 { x: 0, y: 0, z: 0 };
		let vertices = make_cube_data(Vec::new(), show, block, &block_texture);
		println!("{:?}", vertices)
    }
}
// Example usage
