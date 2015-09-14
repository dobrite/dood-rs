
use ai_behavior;

use cascadecs::entity::Entity;

use action::Action;
use dir::Dir;
use path::PathTarget;

pub enum Event {
    None,
    Hunger {
        entity: Entity,
        minus_hunger: u8,
    },
    Movement {
        entity: Entity,
        dir: Dir,
    },
    UpdateBrainState {
        entity: Entity,
        state: ai_behavior::State<Action, ()>,
    },
    PathTo {
        entity: Entity,
        path_target: PathTarget,
    },
    PopPath {
        entity: Entity,
    },
    EatFood {
        entity: Entity,
        target: Entity,
    },
    ComputeFov {
        entity: Entity,
    },
}
