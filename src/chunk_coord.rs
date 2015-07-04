use config;
use loc::Loc;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChunkCoord {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub offset: i32,
}

impl ChunkCoord {
    pub fn new(chunk_x: i32, chunk_y: i32, offset: i32) -> ChunkCoord {
        ChunkCoord {
            chunk_x: chunk_x,
            chunk_y: chunk_y,
            offset: offset
        }
    }

    pub fn from_loc(loc: &Loc) -> ChunkCoord {
        let chunk_x = ((loc.x as f64 / config::CHUNK_WIDTH  as f64)).floor();
        let chunk_y = ((loc.y as f64 / config::CHUNK_HEIGHT as f64)).floor();
        ChunkCoord::new(chunk_x as i32, chunk_y as i32, 0)
    }
}

#[cfg(test)]
mod tests {
    use loc::Loc;
    use chunk_coord::ChunkCoord;

    #[test]
    fn new_it_returns_zero_zero_for_zero_zero() {
        assert!(ChunkCoord::from_loc(&Loc { x: 0, y: 0 }) ==
                ChunkCoord { chunk_x: 0, chunk_y: 0, offset: 0 });
    }

    #[test]
    fn new_it_returns_one_one_for_sixty_five_sixty_five() {
        assert!(ChunkCoord::from_loc(&Loc { x: 65, y: 65 }) ==
                ChunkCoord { chunk_x: 1, chunk_y: 1, offset: 0 });
    }

    #[test]
    fn new_it_returns_one_one_for_sixty_six_sixty_five() {
        assert!(ChunkCoord::from_loc(&Loc { x: 66, y: 65 }) ==
                ChunkCoord { chunk_x: 1, chunk_y: 1, offset: 1 });
    }

    #[test]
    fn new_it_returns_minus_one_minus_one_for_minus_one_minus_one() {
        assert!(ChunkCoord::from_loc(&Loc { x: -1, y: -1 }) ==
                ChunkCoord { chunk_x: -1, chunk_y: -1, offset: 0 });
    }
}
