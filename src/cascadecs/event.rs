
use cascadecs::entity::Entity;

use dir::Dir;

#[derive(Debug)]
pub enum Event {
    Hunger { entity: Entity, minus_hunger: u8 },
    Movement { entity: Entity, dir: Dir }
}
