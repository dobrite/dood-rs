pub use self::camera::Camera;
pub use self::fov::Fov;
pub use self::scratch::Scratch;
pub use self::vertex::Vertex;
pub use self::shaders::{FRAGMENT, VERTEX};
pub use self::flags::{TRANSPARENT, IN_FOV}; // TODO remove this

mod camera;
mod flags;
mod fov;
mod scratch;
mod shaders;
mod vertex;
