use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

use loc::Loc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub loc: Loc,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
