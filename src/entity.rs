use grid::{
    Grid,
};

use loc::{
    Loc,
};

use pixset::{
    Pixset,
};

use renderable::{
    Vertex,
};

pub trait Entity {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex>;
    fn update(&mut self, grid: &Grid, entities: &Vec<Loc>);
}
