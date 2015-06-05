use std::collections::{
    HashMap,
    VecDeque,
};

use pixset::{
    Pix,
    Pixset,
};

use renderable::{
    Renderable,
    Vertex,
};

use updatable::{
    Updatable,
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
    path: Path,
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
            path: HashMap::new(),
        }
    }

}

impl Paths for Dood {
    fn path(&self, grid: &Grid, goal: (i32, i32)) -> Path {
        let mut frontier = VecDeque::new();
        frontier.push_back((self.x, self.y));
        let mut came_from = HashMap::new();
        came_from.insert((self.x, self.y), (-1, -1));

        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();

            if current == goal {
                break
            }

            for next in grid.neighbors(current).iter() {
                if !came_from.contains_key(next) {
                    frontier.push_back(*next);
                    came_from.insert(*next, current);
                }
            }
        }

        return came_from
    }
}

impl Updatable for Dood {
    fn update(&mut self, grid: &Grid) {
        self.hunger -= 0.01;

        // TODO use hunger
        if self.path.is_empty() {
            self.path = self.path(grid, (8, 8));
        }
    }
}

impl Renderable for Dood {
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
