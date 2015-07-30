
use cascadecs::entity::Entity;

use dir::Dir;

#[derive(Debug, PartialEq)]
pub enum Event {
    None,
    Hunger { entity: Entity, minus_hunger: u8 },
    Movement { entity: Entity, dir: Dir }
}
