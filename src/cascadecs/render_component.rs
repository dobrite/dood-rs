
use pixset::Pix;

pub struct RenderComponent {
    pub pix: Pix,
    pub color: [f32; 3],
}

impl RenderComponent {
    pub fn new(pix: Pix, color: [f32; 3]) -> RenderComponent {
        RenderComponent { pix: pix, color: color }
    }
}
