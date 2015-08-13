
use ai_behavior;

use cascadecs::entity;

use action;
use dir;

pub enum Event {
    Hunger {
        entity: entity::Entity,
        minus_hunger: u8,
    },
    Movement {
        entity: entity::Entity,
        dir: dir::Dir,
    },
    UpdateBrainState {
        entity: entity::Entity,
        state: ai_behavior::State<action::Action, ()>,
    }
}
