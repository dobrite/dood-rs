use grid::Grid;
use std::collections::HashMap;

pub type Path = HashMap<(i32, i32), (i32, i32)>;

pub trait Paths {
    fn path(&self, grid: &Grid, goal: (i32, i32)) -> Path;
}
