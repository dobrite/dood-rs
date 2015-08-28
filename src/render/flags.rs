//derives Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord and Hash
bitflags! {
    // attr can be added
    // x: i32,
    flags Flags: u32 {
        const NONE        = 0b00000000,
        const HAS_ENTITY  = 0b00000001,
        const HAS_ITEM    = 0b00000010,
        const PASSABLE    = 0b00000100,
        const TRANSPARENT = 0b00001000,
        const IN_FOV      = 0b00010000,
    }
}
