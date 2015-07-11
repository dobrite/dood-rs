use std::fmt;
use pixset::Pixset;

gfx_vertex!(Vertex {
    vertex_position@ vertex_position: [f32; 2],
    tex_coords@ tex_coords: [f32; 2],
    loc@ loc: [f32; 2],
    color@ color: [f32; 3],
    scale@ scale: f32,
});

pub trait Renderable {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex>;
}

impl fmt::Debug for Renderable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Renderable>")
    }
}
