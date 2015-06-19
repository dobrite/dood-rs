use std::any::Any;
use std::collections::HashMap;

use config::SQUARE_SIZE;
use dood::Dood;
use food::Food;
use loc::Loc;
use grid::Grid;
use wall::Wall;
use entity::Entity;
use window_loc::WindowLoc;

pub struct World {
    pub entities: HashMap<Loc, Box<Any>>,
    grid: Grid,
    blocked: Vec<Loc>,
}

impl World {
    pub fn new() -> World {
        let mut entities = HashMap::new();
        // x, y
        let food_loc = (7, 7);
        let dood_loc = (-8, -8);

        entities.insert(food_loc, Box::new(Food::new(food_loc.0, food_loc.1, SQUARE_SIZE as f32)) as Box<Any>);
        entities.insert(dood_loc, Box::new(Dood::new(dood_loc.0, dood_loc.1, SQUARE_SIZE as f32)) as Box<Any>);

        return World {
            entities: entities,
            grid: Grid::new(16, 16),
            blocked: vec![], // TODO get rid of this
        }
    }

    pub fn spawn(&mut self, window_loc: WindowLoc) {
        let loc = self.grid.to_game_loc(window_loc);
        println!("{} {} {} {}", window_loc.0, window_loc.1, loc.0, loc.1);
        self.entities.insert(loc, Box::new(Food::new(loc.0, loc.1, SQUARE_SIZE as f32)) as Box<Any>);
    }

    pub fn update(&mut self) {
        for (_, entity) in self.entities.iter_mut() {
            match entity.downcast_mut::<Dood>() {
                Some(dood) => dood.update(&self.grid, &self.blocked),
                _ => {}
            }
            match entity.downcast_mut::<Wall>() {
                Some(wall) => wall.update(&self.grid, &self.blocked),
                _ => {}
            }
        }
    }
}
