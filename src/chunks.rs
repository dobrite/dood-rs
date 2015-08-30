
use std::collections::HashMap;

use chunk::Chunk;
use chunk_loc::ChunkLoc;
use size::Size;

pub struct Chunks {
    chunk_size: Size,
    chunks: HashMap<ChunkLoc, Chunk>,
}

impl Chunks {
    pub fn new(chunk_size: Size) -> Chunks {
        let mut chunks = Chunks { chunk_size: chunk_size, chunks: HashMap::new() };

        for x in 0..24 {
            for y in 0..24 {
                // -12, -12, to 11, 11
                chunks.create(ChunkLoc {
                    x: x - 12,
                    y: y - 12,
                });
            }
        }

        chunks
    }

    fn create(&mut self, cl: ChunkLoc) {
        let chunk = Chunk::new(self.chunk_size);
        self.chunks.insert(cl, chunk);
    }

    pub fn get_chunk_mut(&mut self, cl: &ChunkLoc) -> &mut Chunk {
        let chunk_size = self.chunk_size;
        self.chunks.entry(*cl).or_insert_with(|| Chunk::new(chunk_size))
    }
}
