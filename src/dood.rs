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
    PathContour,
    PathContourTrait,
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
            path: Vec::new(),
        }
    }
}

impl Paths for Dood {}

impl Entity for Dood {
    fn update(&mut self, grid: &Grid, entities: &Vec<Loc>) {
        self.hunger -= 0.01;

        // TODO use hunger
        if self.path.is_empty() {
            self.path = self.path(&grid, &entities, (self.x, self.y), (10, 10));
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
