use loc::Loc;

use entity::{
    Entity,
};

pub struct Grid {
    pub height: i32,
    pub width:  i32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid {
        Grid {
            height: height,
            width:  width,
        }
    }

    pub fn in_bounds(&self, loc: &Loc) -> bool {
        let (x, y) = *loc;
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    pub fn passable(&self, loc: &Loc, blocked: &Vec<Loc>) -> bool {
        !blocked.contains(&loc)
    }

    pub fn neighbors(&self, loc: Loc, blocked: &Vec<Loc>) -> Vec<Loc> {
        let (x, y) = loc;
        let results: Vec<Loc> = vec![
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
            (x, y + 1),
        ];
        //if (x + y) % 2 == 0 { results.reverse(); }
        results
            .into_iter()
            .filter(|x| self.in_bounds(x) && self.passable(x, blocked))
            .collect()
    }

    pub fn find(&self, target: &Entity) {
    }
}
