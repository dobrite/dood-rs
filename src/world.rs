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
    chunk_width: i32,
    chunk_height: i32,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    //pub grid: Grid, // TODO prob doesn't need to be in world
}

impl World {
    pub fn new() -> World {
        let mut chunks = HashMap::new();
        let mut world = World {
            chunk_width: CHUNK_WIDTH,
            chunk_height: CHUNK_HEIGHT,
            chunks: chunks,
            //grid: Grid::new(width, height),
        };

        for x in 0..3 {
            for y in 0..3 {
                world.create(ChunkCoord::new(x - 1, y - 1, 0));
            }
        }

        world
    }

    fn create(&mut self, cc: ChunkCoord) {
        let chunk = Chunk::new(self.chunk_width, self.chunk_height);
        self.chunks.insert(cc, chunk);
    }

    fn get_chunk(&mut self, cc: ChunkCoord) -> &Chunk {
        self.chunks.entry(cc).or_insert(Chunk::new(self.chunk_width, self.chunk_height))
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
