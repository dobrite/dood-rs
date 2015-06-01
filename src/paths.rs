use grid::Grid;
use std::collections::HashMap;

pub trait Paths {
    fn path(&self, grid: &Grid, goal: (i32, i32))
        -> HashMap<(i32, i32), (i32, i32)>;
}
