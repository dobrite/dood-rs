use std::collections::HashMap;

use loc::Loc;
use grid::Grid;

pub type Path = HashMap<(i32, i32), (i32, i32)>;

pub trait Paths {
    fn path(&self, grid: &Grid, blocked: &Vec<Loc>, goal: (i32, i32)) -> Vec<Loc>;
}
