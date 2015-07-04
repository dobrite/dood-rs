use terrain::Terrain;

//derives Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord and Hash
bitflags! {
    // attr can be added
    // x: i32,
    flags Flags: u32 {
        const NONE            = 0b00000000,
        const HAS_ENTITY      = 0b00000001,
        const HAS_ITEM        = 0b00000010,
        const BLOCKS_SIGHT    = 0b00000100,
        const BLOCKS_MOVEMENT = 0b00001000,
    }
}

pub struct Scratch {
    terrain: Vec<Terrain>,
    flags: Vec<Flags>,
}

impl Scratch {
    pub fn new(width: i32, height: i32) -> Scratch {
        Scratch {
            terrain: vec![Terrain::None; 3*(width*height) as usize],
            flags: vec![NONE; 3*(width*height) as usize],
        }
    }
}
