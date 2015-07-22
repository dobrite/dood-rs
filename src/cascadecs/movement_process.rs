use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::position_component::PositionComponent;

pub struct MovementProcess {
    components: HashMap<Entity, PositionComponent>
}

impl MovementProcess {
    pub fn new() -> MovementProcess {
        MovementProcess {
            components: HashMap::new(),
        }
    }
}

impl Process for MovementProcess {
    fn process(&self) -> Vec<Event> {
        println!("positioning!");
        vec![]
    }
}
