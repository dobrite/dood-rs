pub trait HasLoc {
    fn loc(&self) -> (i32, i32);
}
