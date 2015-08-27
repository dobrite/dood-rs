
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

//compute_fov(loc.x, loc.y, 10, false);

//for y in &self.transparent {
//    for x in y {
//        if *x {
//            print!(" ")
//        } else {
//            print!(".")
//        }
//    }
//    print!("\n");
//}
//for y in &self.in_fov {
//    for x in y {
//        if *x {
//            print!(" ")
//        } else {
//            print!("X")
//        }
//    }
//    print!("\n");
//}
//
//vec![]
