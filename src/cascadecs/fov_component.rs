
use render::Fov;
use render::{TRANSPARENT, IN_FOV}; // TODO remove this
use size::Size;

pub struct FovComponent {
    pub range: i32,
    pub fov: Fov,
}

impl FovComponent {
    pub fn new(size: Size, range: i32) -> Self {
        FovComponent { range: range, fov: Fov::new(size.width, size.height) }
    }

    pub fn render(&self) {
        let mut ct = 0;
        for flag in &self.fov.flags {
            if flag.contains(TRANSPARENT) {
                print!(" ")
            } else {
                print!(".")
            }
            if ct == self.fov.width {
                print!("\n");
                ct = 0;
            }
        }
        for flag in &self.fov.flags {
            if flag.contains(IN_FOV) {
                print!(" ")
            } else {
                print!("X");
            }
            if ct == self.fov.width {
                print!("\n");
                ct = 0;
            }
        }
    }

}
