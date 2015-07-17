use std::ops::{
    Add,
    Sub,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl Add for Loc {
    type Output = Loc;

    fn add(self, other: Loc) -> Loc {
        Loc { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Loc {
    type Output = Loc;

    fn sub(self, other: Loc) -> Loc {
        Loc { x: self.x - other.x, y: self.y - other.y }
    }
}
