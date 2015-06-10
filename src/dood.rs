use std::collections::{
    HashMap,
    VecDeque,
};

use pixset::{
    Pix,
    Pixset,
};

use loc::{
    Loc,
};

use renderable::{
    Vertex,
};

use entity::{
    Entity,
};

use paths::{
    Path,
    Paths,
};

use grid::Grid;
use config::SQUARE_SIZE;

#[derive(Debug)]
pub struct Dood {
    x: i32,
    y: i32,
    scale: f32,
    color: [f32; 3],
    hunger: f32,
    pix: Pix,
    path: Vec<Loc>, // TODO should maybe redefine Path?
}

impl Dood {
    pub fn new(x: i32, y: i32, square_size: f32) -> Dood {
        return Dood {
            x: x,
            y: y,
            scale: square_size,
            hunger: 100.0,
            color: [0.2; 3],
            pix: Pix::Dood,
            path: Vec::new(),
        }
    }

    // TODO this doesn't belong here
    pub fn construct_path(&self, came_from: Path, goal: Loc) -> Vec<Loc> {
        let mut current = goal;
        let mut path = Vec::new();
        path.push(current);

        while current != (self.x, self.y) {
            match came_from.get(&current) {
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

impl Paths for Dood {
    fn path(&self, grid: &Grid, blocked: &Vec<Loc>, goal: Loc) -> Vec<Loc> {
        let mut frontier = VecDeque::new();
        frontier.push_back((self.x, self.y));
        let mut came_from = HashMap::new();
        came_from.insert((self.x, self.y), (-1, -1));

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

        return self.construct_path(came_from, goal)
    }
}

impl Entity for Dood {
    fn update(&mut self, grid: &Grid, entities: &Vec<Loc>) {
        self.hunger -= 0.01;

        // TODO use hunger
        if self.path.is_empty() {
            // TODO hard coded where food is
            self.path = self.path(&grid, &entities, (10, 10));
        }

        match self.path.pop() {
            Some((x, y)) => {
                self.x = x;
                self.y = y;
            }
            None => {},
        }
    }

    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        implement_vertex!(Vertex, vertex_position, tex_coords, loc, scale, color);

        let offset = (SQUARE_SIZE / 2) as f32;
        let y = (self.y * SQUARE_SIZE) as f32 + offset;
        let x = (self.x * SQUARE_SIZE) as f32 + offset;

        return vec![
            Vertex {
                vertex_position: [-0.5,  0.5],
                tex_coords: tiles.get(&self.pix)[0],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            },  // left  top
            Vertex {
                vertex_position: [ 0.5,  0.5],
                tex_coords: tiles.get(&self.pix)[1],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            },  // right top
            Vertex {
                vertex_position: [ 0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[2],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            }, // right bottom
            Vertex {
                vertex_position: [-0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[3],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            }, // left  bottom
        ]
    }
}
