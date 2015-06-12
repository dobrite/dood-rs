use std::any::Any;
use std::collections::HashMap;

use config::SQUARE_SIZE;
use dood::Dood;
use food::Food;
use loc::Loc;

pub fn gen_world() -> HashMap<Loc, Box<Any>> {
    let mut entities = HashMap::new();
    entities.insert((0, 0), Box::new(Dood::new(0, 0, SQUARE_SIZE as f32)) as Box<Any>);
    //entities.insert((6, 6), Box::new(Wall::new(6, 6, SQUARE_SIZE as f32)) as Box<Any>);
    entities.insert((10, 10), Box::new(Food::new(10, 10, SQUARE_SIZE as f32)) as Box<Any>);
    return entities
}
