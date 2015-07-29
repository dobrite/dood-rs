
#[derive(Clone, Debug)]
pub enum Terrain {
    None,
    Dirt,
    Grass,
    Water,
}

// enum with explicit discriminator
//enum Color {
//    Red = 0xff0000,
//    Green = 0x00ff00,
//    Blue = 0x0000ff,
//}
//http://rustbyexample.com/custom_types/enum/c_like.html
