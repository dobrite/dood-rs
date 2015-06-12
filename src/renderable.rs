use pixset::Pixset;

gfx_vertex!(Vertex {
    vertex_position@ vertex_position: [f32; 2],
    tex_coords@ tex_coords: [f32; 2],
    loc@ loc: [f32; 2],
    color@ color: [f32; 3],
    scale@ scale: f32,
});

impl Vertex {
    fn new(pos: [f32; 2], tc: [f32; 2], loc: [f32; 2], color: [f32; 3], scale: f32) -> Vertex {
        Vertex {
            vertex_position: pos,
            tex_coords: tc,
            loc: loc,
            color: color,
            scale: scale,
        }
    }
}


pub trait Renderable {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex>;
}
