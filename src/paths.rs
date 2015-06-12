use std::collections::{
    HashMap,
    VecDeque,
};

use loc::Loc;
use grid::Grid;

pub type Path = Vec<Loc>;

pub trait Paths {
    fn path(&self, grid: &Grid, blocked: &Vec<Loc>, start: Loc, goal: Loc) -> Path {
        let mut frontier: VecDeque<Loc> = VecDeque::new();
        frontier.push_back(start);
        let mut came_from: PathContour = HashMap::new();
        came_from.insert(start, (-1, -1)); // prob a bad idea

        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();
            if current == goal { break }
            for next in grid.neighbors(current, blocked).iter() {
                if !came_from.contains_key(next) {
                    frontier.push_back(*next);
                    came_from.insert(*next, current);
                }
            }
        }

        return came_from.to_path(start, goal)
    }
}

pub type PathContour = HashMap<Loc, Loc>;

pub trait PathContourTrait {
    fn to_path(&self, from: Loc, to: Loc) -> Path;
}

impl PathContourTrait for PathContour {
    fn to_path(&self, from: Loc, to: Loc) -> Path {
        let mut current = to;
        let mut path = Vec::new();
        path.push(current);

        while current != (from.0, from.1) {
            match self.get(&current) {
                Some(next) => {
                    path.push(*next);
                    current = *next
                }
                None => {}
            }
        }

        return path
    }
}
