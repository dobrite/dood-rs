#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChunkCoord {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub offset: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> ChunkCoord {
        ChunkCoord { chunk_x: x, chunk_y: y, offset: 0 }
    }
}
