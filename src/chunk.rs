use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use rand;

use has_loc::HasLoc;
use loc::Loc;
use food::Food;
use loc_map::LocMap;
use renderable::Renderable;
use terrain::Terrain;

#[derive(Debug)]
pub struct Chunk {
    width: i32,
    height: i32,
    pub terrain: Vec<Terrain>,
    foods: LocMap<Food>,
    renderables: LocMap<Renderable>,
}

impl Chunk {
    pub fn new(width: i32, height: i32) -> Chunk {
        let mut terrain = vec![Terrain::Dirt; (width * height) as usize];

        for terr in &mut terrain {
            if rand::random::<bool>() {
                *terr = Terrain::Grass;
            }
        }

        Chunk {
            width: width,
            height: height,
            terrain: terrain,
            foods: HashMap::new(),
            renderables: HashMap::new(),
        }
    }

    pub fn insert_food(&mut self, loc: Loc, food: Rc<RefCell<Food>>) {
        self.foods.insert(loc, food.clone());
    }

    pub fn insert_renderable(&mut self, loc: Loc, renderable: Rc<RefCell<Renderable>>) {
        self.renderables.insert(loc, renderable.clone());
    }
}
