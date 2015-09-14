
use path::PathTarget;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Idle,
    Content,
    PathTo { path_target: PathTarget },
    EatFood,
}
