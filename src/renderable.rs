#[derive(Debug, Copy, Clone)]
pub struct Render {
    pub loc: [f32; 2],
    pub scale: f32,
    pub color: [f32; 3],
}

impl Render {
    pub fn new(loc: [f32; 2], scale: f32, color: [f32; 3]) -> Render {
        return Render {
            loc: loc,
            scale: scale, // TODO default this to square_size
            color: color,
        }
    }
}

pub trait Renderable {
    fn render(&self) -> Render;
}
