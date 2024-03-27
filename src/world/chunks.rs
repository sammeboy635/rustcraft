use crate::world::chunk::*;

use super::chunk;

pub const NUM_CHUNKS: usize = 16; // Number of chunks in each dimension

struct Chunks {
    chunks: Vec<Vec<Chunk>>,
}


impl Chunks {
    pub fn new() -> Self {
        let chunks = vec![vec![Chunk::new(); NUM_CHUNKS]; NUM_CHUNKS];
        Chunks { chunks }
    }

    pub fn generate(&mut self, seed: u32) {
        for x in 0..NUM_CHUNKS {
            for z in 0..NUM_CHUNKS {
                let mut chunk = Chunk::new();
                chunk.generate(seed);
                self.chunks[x][z] = chunk;
            }
        }
    }

    pub fn get_block(&self, x: isize, y: usize, z: isize) -> Option<Block> {
        let chunk_x = x / CHUNK_WIDTH as isize;
        let chunk_z = z / CHUNK_WIDTH as isize;

        if let Some(chunk) = self.chunks.get(chunk_x as usize).and_then(|row| row.get(chunk_z as usize)) {
            let block_x = (x % CHUNK_WIDTH as isize) as usize;
            let block_z = (z % CHUNK_WIDTH as isize) as usize;
            chunk.get_block(block_x, y, block_z)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_chunks_gen() {
        // println!("test");
        let mut chunks = Chunks::new();
        chunks.generate(123); // Seed for random generation
        
        println!("{:?}", chunks.get_block(52, 0, 52)); 
    }
}