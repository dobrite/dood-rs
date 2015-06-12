use pixset::{
    Pix,
    Pixset,
};

use config::SQUARE_SIZE;
use entity::Entity;
use grid::Grid;
use loc::Loc;
use renderable::Vertex;

#[derive(Debug)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub color: [f32; 3],
    pub pix: Pix,
}

impl Wall {
    pub fn new(x: i32, y: i32, square_size: f32) -> Wall {
        return Wall {
            x: x,
            y: y,
            scale: square_size,
            color: [0.0, 0.0, 0.0],
            pix: Pix::DownArrow,
        }
    }
}

impl Entity for Wall {
    fn update(&mut self, _: &Grid, _: &Vec<Loc>) {}

    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (SQUARE_SIZE / 2) as f32;
        let y = (self.y * SQUARE_SIZE) as f32 + offset;
        let x = (self.x * SQUARE_SIZE) as f32 + offset;

        return vec![
            Vertex {
                vertex_position: [-0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[0],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },  // left bottom
            Vertex {
                vertex_position: [0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[1],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },  // right bottom
            Vertex {
                vertex_position: [0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[2],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            }, // right top
            Vertex {
                vertex_position: [-0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[3],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            }, // left  top
        ]
    }
}
