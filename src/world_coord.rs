
use config;

use chunk_loc::ChunkLoc;
use loc::Loc;
use size::Size;
use indices::Indices;

///  ### Cartesian Coords vs. Loc
///  upper left cart coord is world coord of square
///
///           -1,1      |      1,1
///           - - - - - - - - - - -
///             |       |       |
/// (cc -1,1)   | -1,1  |  0,1  | (cc 0,1)
///             |       |       |
///           ---------0,0---------
///             |       |       |
/// (cc -1,0)   | -1,0  |  0,0  | (cc 0,0)
///             |       |       |
///           - - - - - - - - - - -
///           -1,-1     |      1,-1
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorldCoord {
    size: Size,
    chunk_loc: ChunkLoc,
    indices: Indices,
}

impl WorldCoord {
    fn new(size: Size, chunk_loc: ChunkLoc, indices: Indices) -> WorldCoord {
        WorldCoord { size: size, chunk_loc: chunk_loc, indices: indices }
    }

    pub fn from_loc(loc: &Loc) -> WorldCoord {
        let size = WorldCoord::get_size();
        let chunk_x = chunk(loc.x, size.width);
        let chunk_y = chunk(loc.y - 1, size.height) + 1;
        let row = modulo(size.height - loc.y, size.height);
        let col = modulo(loc.x, size.width);
        WorldCoord::new(size,
                        ChunkLoc { x: chunk_x, y: chunk_y },
                        Indices::new(row, col, size.width))
    }

    pub fn from_chunk_loc(chunk_loc: &ChunkLoc) -> WorldCoord {
        WorldCoord::new(WorldCoord::get_size(), *chunk_loc, Indices::new(0, 0, config::CHUNK_WIDTH))
    }

    pub fn to_loc(&self) -> Loc {
        let x = self.size.width * self.chunk_loc.x + self.indices.col;
        let y = self.size.height * self.chunk_loc.y - self.indices.row;
        Loc { x: x, y: y }
    }

    pub fn get_chunk_loc(&self) -> ChunkLoc {
        self.chunk_loc
    }

    fn get_size() -> Size {
        Size { width: config::CHUNK_WIDTH, height: config::CHUNK_HEIGHT }
    }
}

fn chunk(a: i32, dim: i32) -> i32 {
    (a as f64 / dim as f64).floor() as i32
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[cfg(test)]
mod tests {
    use chunk_loc::ChunkLoc;
    use loc::Loc;
    use size::Size;
    use indices::Indices;
    use world_coord::WorldCoord;

    #[test]
    fn to_loc_it_round_trip_for_0_0() {
        let loc = Loc { x: 0, y: 0 };
        assert!(WorldCoord::from_loc(&loc).to_loc() == loc)
    }

    #[test]
    fn to_loc_it_round_trip_for_16_16() {
        let loc = Loc { x: 16, y: 16 };
        assert!(WorldCoord::from_loc(&loc).to_loc() == loc)
    }

    #[test]
    fn to_loc_it_round_trip_for_1_1() {
        let loc = Loc { x: 1, y: 1 };
        assert!(WorldCoord::from_loc(&loc).to_loc() == loc)
    }

    #[test]
    fn to_loc_it_round_trip_for_minus_47_120() {
        let loc = Loc { x: -47, y: 120 };
        assert!(WorldCoord::from_loc(&loc).to_loc() == loc)
    }

    // Upper Right Quadrant

    #[test]
    fn from_loc_16_16_it_returns_correct_for_0_1() {
        let loc = Loc { x: 0, y: 1 };
        let cl = ChunkLoc { x: 0, y: 1 };
        let indices = Indices { row: 15, col: 0, width: 16 };
        let wc = WorldCoord { size: WorldCoord::get_size(), chunk_loc: cl, indices: indices };
        assert!(WorldCoord::from_loc(&loc) == wc);
    }

    #[test]
    fn from_loc_16_16_it_returns_correct_for_16_17() {
        let loc = Loc { x: 16, y: 17 };
        let cl = ChunkLoc { x: 1, y: 2 };
        let indices = Indices { row: 15, col: 0, width: 16 };
        let wc = WorldCoord { size: WorldCoord::get_size(), chunk_loc: cl, indices: indices };
        assert!(WorldCoord::from_loc(&loc) == wc);
    }

    // Lower Left Quadrant

    #[test]
    fn from_loc_16_16_it_returns_correct_for_minus_17_minus_16() {
        let loc = Loc { x: -17, y: -16 };
        let cl = ChunkLoc { x: -2, y: -1 };
        let indices = Indices { row: 0, col: 15, width: 16 };
        let wc = WorldCoord { size: WorldCoord::get_size(), chunk_loc: cl, indices: indices };
        assert!(WorldCoord::from_loc(&loc) == wc);
    }
}
