
use path::{Path, PathTarget};

pub struct PathComponent {
    pub path: Path,
    // TODO check if this is used
    pub target: PathTarget,
}

impl PathComponent {
    pub fn new(path: Path, target: PathTarget) -> Self {
        PathComponent { path: path, target: target }
    }
}
