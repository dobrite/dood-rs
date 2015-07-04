use std::collections::HashMap;

use loc::Loc;
use terrain::Terrain;
use has_loc::HasLoc;

#[derive(Debug)]
pub struct Chunk {
    width: i32,
    height: i32,
    terrain: Vec<Terrain>,
    entities: HashMap<Loc, Box<HasLoc>>,
}

impl Chunk {
    pub fn new(width: i32, height: i32) -> Chunk {
        Chunk {
            width: width,
            height: height,
            terrain: vec![Terrain::Dirt; (width*height) as usize],
            entities: HashMap::new(),
        }
    }
}
