use dood::Dood;
use food::Food;
use wall::Wall;
use renderable::Renderable;

pub struct Grid {
    pub stuffs: Vec<Box<Renderable + Send>>,
    pub walls:  Vec<(i32, i32)>,
    pub height: i32,
    pub width:  i32,
    pub dood: Dood,
}

impl Grid {
    pub fn new(width: i32, height: i32, square_size: f32) -> Grid {

        let mut stuffs: Vec<Box<Renderable + Send>> = Vec::new();

        //TODO: dumb
        let d = Dood::new(4, 4, square_size);
        //*tiles.tiles.get(&Pix::Dood).unwrap()

        stuffs.push(Box::new(Dood::new(4, 4, square_size)));
        stuffs.push(Box::new(Food {
            x:  8, //  520   16 * 32 + (16 / 2)
            y:  8, // -520 -(16 * 32 + (16 / 2))
            scale: square_size,
            color: [0.2313725, 0.3254902, 0.1372549]
        }));

        stuffs.push(Box::new(Wall {
            x:  6, //  520   16 * 32 + (16 / 2)
            y:  6, // -520 -(16 * 32 + (16 / 2))
            scale: square_size,
            color: [0.0, 0.0, 0.0]
        }));

        let mut walls = Vec::new();
        walls.push((6, 6));

        Grid {
            stuffs: stuffs,
            height: height,
            width:  width,
            walls:  walls,
            dood:   d,
        }
    }

    pub fn update(&self) {
        return
        //for stuff in self.stuffs.iter_mut() {
        //    stuff.update();
        //}
    }

    pub fn in_bounds(&self, loc: &(i32, i32)) -> bool {
        let (x, y) = *loc;
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    pub fn passable(&self, loc: &(i32, i32)) -> bool {
        !self.walls.contains(&loc)
    }

    pub fn neighbors(&self, loc: (i32, i32)) -> Vec<(i32, i32)> {
        let (x, y) = loc;
        let results: Vec<(i32, i32)> = vec![
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
            (x, y + 1),
        ];
        //if (x + y) % 2 == 0 { results.reverse(); }
        results
            .into_iter()
            .filter(|x| self.in_bounds(x) && self.passable(x))
            .collect()
    }
}
