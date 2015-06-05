use std::sync::{
    Arc,
    Mutex,
};

use dood::Dood;
use food::Food;
use wall::Wall;
use pixset::Pix;
use renderable::Renderable;
use updatable::Updatable;

pub struct Grid {
    pub updatables: Vec<Arc<Mutex<Box<Updatable+Send+Sync>>>>,
    pub renderables: Vec<Arc<Mutex<Box<Renderable+Send+Sync>>>>,
    pub walls:  Vec<(i32, i32)>,
    pub height: i32,
    pub width:  i32,
}

impl Grid {
    pub fn new(width: i32, height: i32, square_size: f32) -> Grid {
        let mut updatables: Vec<Arc<Mutex<Box<Updatable+Send+Sync>>>> = Vec::new();
        let mut renderables: Vec<Arc<Mutex<Box<Renderable+Send+Sync>>>> = Vec::new();

        //let d = Box::new(Dood::new(4, 4, square_size));
        //                 Arc::new(Mutex::new());

        //updatables.push(d.clone());

        //renderables.push(d.clone());
        let d = Arc::new(Mutex::new(Box::new(Dood::new(4, 4, square_size)) as Box<Renderable+Send+Sync>));
        renderables.push(d.clone());

        let w = Arc::new(Mutex::new(Box::new(Wall::new(6, 6, square_size)) as Box<Renderable+Send+Sync>));
        renderables.push(w.clone());

        let f = Arc::new(Mutex::new(Box::new(Food::new(8, 8, square_size)) as Box<Renderable+Send+Sync>));
        renderables.push(f.clone());

        let mut walls = Vec::new();
        walls.push((6, 6));

        Grid {
            renderables: renderables,
            updatables: updatables,
            height: height,
            width:  width,
            walls:  walls,
        }
    }

    pub fn update(&self) {
        for updatable in self.updatables.iter() {
            updatable.lock().unwrap().update(self);
        }
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
