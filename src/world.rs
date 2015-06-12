use std::any::Any;
use std::collections::HashMap;

use config::SQUARE_SIZE;
use dood::Dood;
use food::Food;
use loc::Loc;
use grid::Grid;
use wall::Wall;
use entity::Entity;

pub struct World {
    pub entities: HashMap<Loc, Box<Any>>,
    grid: Grid,
    blocked: Vec<Loc>,
}

impl World {
    pub fn new() -> World {
        let mut entities = HashMap::new();
        entities.insert((0, 0), Box::new(Dood::new(0, 0, SQUARE_SIZE as f32)) as Box<Any>);
        //entities.insert((6, 6), Box::new(Wall::new(6, 6, SQUARE_SIZE as f32)) as Box<Any>);
        entities.insert((10, 10), Box::new(Food::new(10, 10, SQUARE_SIZE as f32)) as Box<Any>);

        // TODO only do walls (not food and player)
        //let blocked = entities.keys().cloned().collect::<Vec<_>>();
        let blocked = vec![];

        return World {
            entities: entities,
            grid: Grid::new(16, 16),
            blocked: blocked, // TODO get rid of this
        }
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
