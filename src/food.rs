use std::collections::HashMap;

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

use grid::Grid;

#[derive(Debug)]
pub struct Food {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub color: [f32; 3],
    pub pix: Pix,
}

impl Food {
    pub fn new(x: i32, y: i32, square_size: f32) -> Food {
        return Food {
            x: x,
            y: y,
            scale: square_size,
            color: [0.2313725, 0.3254902, 0.1372549],
            pix: Pix::UpArrow,
        }
    }
}

impl Updatable for Food {
    fn update(&mut self, grid: &Grid) {}
}

impl Renderable for Food {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let y = self.y as f32 * 16.0 + 8.0;
        let x = self.x as f32 * 16.0 + 8.0;
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
