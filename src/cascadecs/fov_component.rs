
use render::Fov;
use size::Size;

pub struct FovComponent {
    pub range: i32,
    pub fov: Fov,
}

impl FovComponent {
    pub fn new(size: Size, range: i32) -> Self {
        FovComponent { range: range, fov: Fov::new(size.width, size.height) }
    }
}
