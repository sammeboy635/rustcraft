extern crate noise;

use noise::{NoiseFn, Perlin, Seedable};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
#[derive(Copy, Clone)]
pub struct Chunk {
    blocks: [[[Block; CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_WIDTH],
}

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Air,
    Stone,
    Grass,
    Dirt,
    // Add more block types as needed
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Block::Air; CHUNK_HEIGHT]; CHUNK_WIDTH]; CHUNK_WIDTH],
        }
    }

    pub fn generate(&mut self, seed: u32) {
        let perlin = Perlin::new(seed);
        // perlin.set_seed(seed);

        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let height = ((perlin.get([x as f64 / 16.0, z as f64 / 16.0]) + 1.0) * 32.0) as usize;
                // println!("{:?}", height);
                for y in 0..height {
                    self.blocks[x][z][y] = match y {
                        h if h >= 128 => Block::Stone,
                        h if h >= 120 => Block::Dirt,
                        h if h == height => Block::Grass,
                        h if h >= height - 3 => Block::Dirt,
                        _ => Block::Stone,
                    };
                }
            }
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<Block> {
        if x < CHUNK_WIDTH && y < CHUNK_HEIGHT && z < CHUNK_WIDTH {
            Some(self.blocks[x][z][y])
        } else {
            None
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) -> Option<()> {
        if x < CHUNK_WIDTH && y < CHUNK_HEIGHT && z < CHUNK_WIDTH {
            self.blocks[x][z][y] = block;
            Some(())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_chunk_gen() {
		let mut chunk = Chunk::new();
        chunk.generate(123); // Seed for random generation

        println!("{:?}", chunk.get_block(0, 0, 0)); // E
    }
}

