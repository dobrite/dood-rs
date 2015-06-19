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
pub struct Food {
    pub loc: Loc,
    pub scale: f32,
    pub color: [f32; 3],
    pub pix: Pix,
}

impl Food {
    pub fn new(loc: Loc, square_size: f32) -> Food {
        return Food {
            loc: loc,
            scale: square_size,
            color: [0.2313725, 0.3254902, 0.1372549],
            pix: Pix::UpArrow,
        }
    }
}

impl Entity for Food {
    fn update(&mut self, _: &Grid, _: &Vec<Loc>) {}

    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (SQUARE_SIZE / 2) as f32;
        let x = (self.loc.0 * SQUARE_SIZE as u32) as f32 + offset;
        let y = (self.loc.1 * SQUARE_SIZE as u32) as f32 + offset;

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
