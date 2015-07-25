
use std::fmt;

use loc::Loc;

pub trait HasLoc {
    fn get_loc(&self) -> Loc;
}

impl fmt::Debug for HasLoc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get_loc())
    }
}
