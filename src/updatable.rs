use grid::Grid;

trait Updatable2: Updatable + Send + Sync {}

pub trait Updatable {
    fn update(&mut self, grid: &Grid);
}
