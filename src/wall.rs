use renderable::{Render, Renderable};

#[derive(Debug)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub color: [f32; 3],
}

impl Renderable for Wall {
    fn render(&self) -> Render {
        let y = self.y as f32 * 16.0 + 8.0;
        let x = self.x as f32 * 16.0 + 8.0;
        return Render {
            loc: [x, -y],
            scale: self.scale,
            color: self.color,
        }
    }
}
