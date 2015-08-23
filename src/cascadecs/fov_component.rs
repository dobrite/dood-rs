
use fov::Fov;

pub struct FovComponent {
    pub range: i32,
}

impl FovComponent {
    pub fn new(range: i32) -> Self {
        FovComponent { range: range }
    }
}

//compute_fov(loc.x, loc.y, 10, false);

//        for y in &self.transparent {
//            for x in y {
//                if *x {
//                    print!(" ")
//                } else {
//                    print!(".")
//                }
//            }
//            print!("\n");
//        }
//        for y in &self.in_fov {
//            for x in y {
//                if *x {
//                    print!(" ")
//                } else {
//                    print!("X")
//                }
//            }
//            print!("\n");
//        }
//
//        vec![]
