
use size::Size;
use loc::Loc;

pub struct Grid {
    pub size: Size,
}

impl Grid {
    pub fn new(size: Size) -> Grid {
        Grid { size: size }
    }

    pub fn in_bounds(&self, _: &Loc) -> bool {
        // TODO fix
        //let (x, y) = *loc;
        //0 <= x && x < self.width && 0 <= y && y < self.height
        true
    }

    pub fn passable(&self, loc: &Loc, blocked: &[Loc]) -> bool {
        !blocked.contains(&loc)
    }

    pub fn neighbors(&self, loc: Loc, blocked: &[Loc]) -> Vec<Loc> {
        let x = loc.x;
        let y = loc.y;

        let results: Vec<Loc> = vec![
            Loc { x: x + 1, y: y     },
            Loc { x: x,     y: y - 1 },
            Loc { x: x - 1, y: y     },
            Loc { x: x,     y: y + 1 },
        ];
        //if (x + y) % 2 == 0 { results.reverse(); }
        results
            .into_iter()
            .filter(|x| self.in_bounds(x) && self.passable(x, blocked))
            .collect()
    }
}
