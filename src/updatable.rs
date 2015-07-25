
use world::World;

pub trait Updatable {
    fn update(&mut self, world: &World);
}
