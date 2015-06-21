use grid::Grid;
use loc::Loc;

pub trait Updatable {
    fn update(&mut self, grid: &Grid, entities: &Vec<Loc>);
}
