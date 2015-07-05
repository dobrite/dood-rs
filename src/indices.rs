#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Indices {
    pub row: i32,
    pub col: i32,
}

impl Indices {
    pub fn new(row: i32, col: i32) -> Indices {
        Indices {
            row: row,
            col: col,
        }
    }
}
