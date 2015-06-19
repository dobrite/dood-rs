use std::any::Any;
use std::collections::HashMap;

use config::SQUARE_SIZE;

use dood::Dood;
use entity::Entity;
use food::Food;
use grid::Grid;
use loc::Loc;
use window_loc::WindowLoc;

pub struct World {
    pub entities: HashMap<Loc, Box<Any>>,
    grid: Grid,
    blocked: Vec<Loc>,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut entities = HashMap::new();
        // x, y
        let food_loc = (0, 0);
        let dood_loc = (31, 31);

        entities.insert(food_loc, Box::new(Food::new(food_loc, SQUARE_SIZE as f32)) as Box<Any>);
        entities.insert(dood_loc, Box::new(Dood::new(dood_loc, SQUARE_SIZE as f32)) as Box<Any>);

        return World {
            entities: entities,
            grid: Grid::new(width, height),
            blocked: vec![], // TODO get rid of this
        }
    }

    pub fn spawn(&mut self, window_loc: WindowLoc) {
        let loc = self.grid.to_game_loc(window_loc);
        println!("{} {} {} {}", window_loc.0, window_loc.1, loc.0, loc.1);
        self.entities.insert(loc, Box::new(Food::new(loc, SQUARE_SIZE as f32)) as Box<Any>);
    }

    pub fn update(&mut self) {
        for (_, entity) in self.entities.iter_mut() {
            match entity.downcast_mut::<Dood>() {
                Some(dood) => dood.update(&self.grid, &self.blocked),
                _ => {}
            }
        }
    }
}
