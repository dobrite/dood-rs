use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::position_component::PositionComponent;

pub struct PositionProcess {
    components: HashMap<Entity, PositionComponent>
}

impl PositionProcess {
    pub fn new() -> PositionProcess {
        components: HashMap::new(),
    }
}

impl Process for PositionProcess {
    fn process(&self) -> Vec<Event> {
        println!("positioning!");
        vec![]
    }
}
