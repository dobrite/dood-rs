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

// TODO make a WorldCoord Factory that knows size

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorldCoord {
    size: Size,
    chunk_loc: ChunkLoc,
    indices: Indices,
}

impl WorldCoord {
    fn new(size: Size, chunk_loc: ChunkLoc, indices: Indices) -> WorldCoord {
        WorldCoord {
            size: size,
            chunk_loc: chunk_loc,
            indices: indices,
        }
    }

    pub fn from_loc(size: &Size, loc: &Loc) -> WorldCoord {
        let chunk_x = chunk(loc.x, size.width);
        let chunk_y = chunk(loc.y - 1, size.height) + 1;
        let row = modulo(size.height - loc.y, size.height);
        let col = modulo(loc.x, size.width);
        WorldCoord::new(*size, ChunkLoc { x: chunk_x, y: chunk_y }, Indices::new(row, col))
    }

    pub fn get_chunk_loc(&self) -> ChunkLoc {
        self.chunk_loc
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

    // Upper Right Quadrant

    #[test]
    fn from_loc_it_returns_correct_for_0_1() {
        let loc = Loc { x: 0, y: 1 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 7, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_1_1() {
        let loc = Loc { x: 1, y: 1 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 7, col: 1 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_0_2() {
        let loc = Loc { x: 0, y: 2 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 6, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_0_8() {
        let loc = Loc { x: 0, y: 8 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 0, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_7_8() {
        let loc = Loc { x: 7, y: 8 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 0, col: 7 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_7_1() {
        let loc = Loc { x: 7, y: 1 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 7, col: 7 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_8_1() {
        let loc = Loc { x: 8, y: 1 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: 1 }, indices: Indices { row: 7, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_8_9() {
        let loc = Loc { x: 8, y: 9 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: 2 }, indices: Indices { row: 7, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_15_16() {
        let loc = Loc { x: 15, y: 16 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: 2 }, indices: Indices { row: 0, col: 7 } });
    }

    // Lower Right Quadrant

    #[test]
    fn from_loc_it_returns_correct_for_0_0() {
        let loc = Loc { x: 0, y: 0 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 0 }, indices: Indices { row: 0, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_8_minus_8() {
        let loc = Loc { x: 8, y: -8 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: -1 }, indices: Indices { row: 0, col: 0 } });
    }

    #[test]
    fn from_loc_it_returns_correct_for_15_minus_15() {
        let loc = Loc { x: 15, y: -15 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: -1 }, indices: Indices { row: 7, col: 7 } });
    }

    // Upper Left Quadrant

    #[test]
    fn from_loc_it_returns_correct_for_minus_1_1() {
        let loc = Loc { x: -1, y: 1 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: -1, y: 1 }, indices: Indices { row: 7, col: 7 } });
    }

    // Lower Left Quadrant

    #[test]
    fn from_loc_it_returns_correct_for_minus_1_0() {
        let loc = Loc { x: -1, y: 0 };
        let size = Size { width: 8, height: 8 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: -1, y: 0 }, indices: Indices { row: 0, col: 7 } });
    }

    // 16x16
    // Upper Right Quadrant

    #[test]
    fn from_loc_16_16_it_returns_correct_for_0_1() {
        let loc = Loc { x: 0, y: 1 };
        let size = Size { width: 16, height: 16 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 0, y: 1 }, indices: Indices { row: 15, col: 0 } });
    }

    #[test]
    fn from_loc_16_16_it_returns_correct_for_16_17() {
        let loc = Loc { x: 16, y: 17 };
        let size = Size { width: 16, height: 16 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: 1, y: 2 }, indices: Indices { row: 15, col: 0 } });
    }

    // Lower Left Quadrant

    #[test]
    fn from_loc_16_16_it_returns_correct_for_minus_17_minus_16() {
        let loc = Loc { x: -17, y: -16 };
        let size = Size { width: 16, height: 16 };
        assert!(WorldCoord::from_loc(&size, &loc) ==
                WorldCoord { size: size, chunk_loc: ChunkLoc { x: -2, y: -1 }, indices: Indices { row: 0, col: 15 } });
    }
}
