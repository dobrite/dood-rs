use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{
    Rc,
    Weak,
};

use config::{
    CHUNK_WIDTH,
    CHUNK_HEIGHT,
};

use chunk::Chunk;
use chunk_coord::ChunkCoord;
use dood::Dood;
use food::Food;
use fov::Fov;
use grid::Grid;
use has_loc::HasLoc;
use loc::Loc;
use loc_map::LocMap;
use renderable::Renderable;
use updatable::Updatable;
use wall::Wall;

pub struct World {
    pub chunks: HashMap<ChunkCoord, Chunk>,
    //pub grid: Grid, // TODO prob doesn't need to be in world
}

impl World {
    pub fn new() -> World {
        let mut chunks = HashMap::new();

        for x in 0..3 {
            for y in 0..3 {
                chunks.insert(ChunkCoord { x: x - 1, y: y - 1 }, Chunk::new(CHUNK_WIDTH, CHUNK_HEIGHT));
            }
        }

        World {
            chunks: chunks,
            //grid: Grid::new(width, height),
        }
    }

    pub fn spawn_food(&mut self, loc: Loc) {
        //let food = Rc::new(RefCell::new(Food::new(loc)));
        //self.renderables.insert(loc, food.clone() as Rc<RefCell<Renderable>>);
        //self.foods.insert(loc, food.clone());
    }

    pub fn spawn_wall(&mut self, loc: Loc) {
        //let wall = Rc::new(RefCell::new(Wall::new(loc)));
        //self.renderables.insert(loc, wall.clone() as Rc<RefCell<Renderable>>);
        //self.walls.insert(loc, wall.clone());
        //for fov in &self.fovs {
        //    fov.borrow_mut().set_transparent(loc.x, loc.y, false)
        //}
    }

    pub fn update(&self) {
        //for (_, entity) in self.updatables.iter() {
        //    entity.borrow_mut().update(self);
        //}
    }

    pub fn vacuum(&mut self) {
        //let mut remove = vec![];
        //self.to_remove(&mut remove);
        //for loc in remove.iter() {
        //    self.foods.remove(loc);
        //    self.renderables.remove(loc);
        //    self.updatables.remove(loc);
        //}
    }

    fn to_remove(&self, remove: &mut Vec<Loc>) {
        //for (loc, food) in self.foods.iter() {
        //    if food.borrow().get_noms() <= 0.0 {
        //        remove.push(*loc);
        //    }
        //}
    }
}

fn world_loc_to_chunk_loc(loc: Loc) -> ChunkCoord {
    ChunkCoord { x: 0, y: 0 }
}

#[cfg(test)]
mod tests {
    use loc::Loc;
    use chunk_coord::ChunkCoord;
    use super::world_loc_to_chunk_loc;

    #[test]
    fn new_it_returns_zero_zero_for_zero_zero() {
        assert!(world_loc_to_chunk_loc(Loc { x: 0, y: 0 }) == ChunkCoord { x: 0, y: 0 });
    }
}
