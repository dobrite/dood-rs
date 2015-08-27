
use render::Vertex;
use pixset::{Pix, Pixset};

pub struct RenderComponent {
    pub pix: Pix,
    pub color: [f32; 3],
}

impl RenderComponent {
    pub fn new(pix: Pix, color: [f32; 3]) -> RenderComponent {
        RenderComponent { pix: pix, color: color }
    }

    pub fn render(&self, x: f32, y: f32, vertex_data: &mut Vec<Vertex>, tiles: &Pixset) {
        vertex_data.push(Vertex {
            vertex_position: [0.0, 0.0],
            tex_coords: tiles.get(&self.pix)[0],
            loc: [x, y],
            scale: 16.0,
            color: self.color,
        });
        vertex_data.push(Vertex {
            vertex_position: [1.0, 0.0],
            tex_coords: tiles.get(&self.pix)[1],
            loc: [x, y],
            scale: 16.0,
            color: self.color,
        });
        vertex_data.push(Vertex {
            vertex_position: [1.0, 1.0],
            tex_coords: tiles.get(&self.pix)[2],
            loc: [x, y],
            scale: 16.0,
            color: self.color,
        });
        vertex_data.push(Vertex {
            vertex_position: [0.0, 1.0],
            tex_coords: tiles.get(&self.pix)[3],
            loc: [x, y],
            scale: 16.0,
            color: self.color,
        });
    }
}
