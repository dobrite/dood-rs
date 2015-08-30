
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Indices {
    pub row: i32, // TODO makes these usize?
    pub col: i32,
    pub width: i32,
}

impl Indices {
    pub fn new(row: i32, col: i32, width: i32) -> Indices {
        Indices { row: row, col: col, width: width }
    }

    pub fn to_1d(&self) -> usize {
        (self.width * self.row + self.col) as usize
    }
}
