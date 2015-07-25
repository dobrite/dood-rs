
use std::collections::HashMap;

use chunk::Chunk;
use chunk_loc::ChunkLoc;
use size::Size;

pub struct Chunks {
    chunk_size: Size,
    chunks: HashMap<ChunkLoc, Chunk>,
}

impl Chunks {
    pub fn new(chunk_size: Size) -> Chunks {
        let mut chunks = Chunks { chunk_size: chunk_size, chunks: HashMap::new() };

        for x in 0..24 {
            for y in 0..24 {
                // -12, -12, to 11, 11
                chunks.create(ChunkLoc {
                    x: x - 12,
                    y: y - 12,
                });
            }
        }

        chunks
    }

    fn create(&mut self, cl: ChunkLoc) {
        let chunk = Chunk::new(self.chunk_size);
        self.chunks.insert(cl, chunk);
    }

    pub fn get_chunk(&mut self, cl: &ChunkLoc) -> &mut Chunk {
        let chunk_size = self.chunk_size;
        self.chunks.entry(*cl).or_insert_with(|| Chunk::new(chunk_size))
    }

    //pub fn spawn_wall(&mut self, loc: Loc) {
        //let wall = Rc::new(RefCell::new(Wall::new(loc)));
        //self.renderables.insert(loc, wall.clone() as Rc<RefCell<Renderable>>);
        //self.walls.insert(loc, wall.clone());
        //for fov in &self.fovs {
        //    fov.borrow_mut().set_transparent(loc.x, loc.y, false)
        //}
    //}

    //pub fn update(&self) {
        //for (_, entity) in self.updatables.iter() {
        //    entity.borrow_mut().update(self);
        //}
    //}

    //pub fn vacuum(&mut self) {
        //let mut remove = vec![];
        //self.to_remove(&mut remove);
        //for loc in remove.iter() {
        //    self.foods.remove(loc);
        //    self.renderables.remove(loc);
        //    self.updatables.remove(loc);
        //}
    //}

    //fn to_remove(&self, remove: &mut &[Loc]) {
        //for (loc, food) in self.foods.iter() {
        //    if food.borrow().get_noms() <= 0.0 {
        //        remove.push(*loc);
        //    }
        //}
    //}
}