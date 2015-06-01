use std::collections::HashMap;

use pixset::{
    Pix,
    Pixset,
};

use renderable::{
    Renderable,
    Vertex,
};

#[derive(Debug)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub color: [f32; 3],
    pub pix: Pix,
}

impl Renderable for Wall {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let y = self.y as f32 * 16.0 + 8.0;
        let x = self.x as f32 * 16.0 + 8.0;
        return vec![
            Vertex {
                vertex_position: [-0.5,  0.5],
                tex_coords: tiles.tiles.get(&self.pix).unwrap()[0],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            },  // left  top
            Vertex {
                vertex_position: [ 0.5,  0.5],
                tex_coords: tiles.tiles.get(&self.pix).unwrap()[1],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            },  // right top
            Vertex {
                vertex_position: [ 0.5, -0.5],
                tex_coords: tiles.tiles.get(&self.pix).unwrap()[2],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            }, // right bottom
            Vertex {
                vertex_position: [-0.5, -0.5],
                tex_coords: tiles.tiles.get(&self.pix).unwrap()[3],
                loc: [x, -y],
                scale: self.scale,
                color: self.color
            }, // left  bottom
        ]
    }
}
