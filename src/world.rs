use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{
    Rc,
    Weak,
};

use config;
use chunk::Chunk;
use chunk_loc::ChunkLoc;
use food::Food;
use fov::Fov;
use has_loc::HasLoc;
use loc::Loc;
use loc_map::LocMap;
use renderable::Renderable;
use size::Size;
use updatable::Updatable;
use wall::Wall;
use world_coord::WorldCoord;

pub struct World {
    chunk_size: Size,
    pub chunks: HashMap<ChunkLoc, Chunk>,
}

impl World {
    pub fn new(chunk_size: Size) -> World {
        let mut chunks = HashMap::new();
        let mut world = World {
            chunk_size:  chunk_size,
            chunks: chunks,
        };

        for x in 0..24 {
            for y in 0..24 {
                // -12, -12, to 11, 11
                world.create(ChunkLoc {
                    x: x - 12,
                    y: y - 12,
                });
            }
        }

        world
    }

    fn create(&mut self, cl: ChunkLoc) {
        let chunk = Chunk::new(self.chunk_size);
        self.chunks.insert(cl, chunk);
    }

    pub fn get_chunk(&mut self, cl: &ChunkLoc) -> &mut Chunk {
        let chunk_size = self.chunk_size;
        self.chunks.entry(*cl).or_insert_with(|| Chunk::new(chunk_size))
    }

    pub fn spawn_food(&mut self, loc: Loc) {
        let size = Size { width: 16, height: 16 }; // TODO fix with WorldCoordFactory or some such
        let ref mut chunk = *self.get_chunk(&WorldCoord::from_loc(&size, &loc).get_chunk_loc());
        let food = Rc::new(RefCell::new(Food::new(loc, config::SQUARE_SIZE))); // TODO get rid of config
        chunk.insert_food(loc, food.clone());
        chunk.insert_renderable(loc, food.clone() as Rc<RefCell<Renderable>>);
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

    fn to_remove(&self, remove: &mut &[Loc]) {
        //for (loc, food) in self.foods.iter() {
        //    if food.borrow().get_noms() <= 0.0 {
        //        remove.push(*loc);
        //    }
        //}
    }
}
