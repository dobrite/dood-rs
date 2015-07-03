use std::fmt;

use loc::Loc;

pub trait HasLoc {
    fn get_loc(&self) -> Loc;
}
