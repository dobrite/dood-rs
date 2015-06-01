use std::collections::HashMap;

use pixset::{
    Pix,
    Pixset,
};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub vertex_position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub loc: [f32; 2],
    pub color: [f32; 3],
    pub scale: f32,
}

pub trait Renderable {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex>;
}
