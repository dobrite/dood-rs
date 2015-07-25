
use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::hunger_component::HungerComponent;

pub struct HungerProcess {
    components: HashMap<Entity, HungerComponent>,
}

impl HungerProcess {
    pub fn new() -> HungerProcess {
        HungerProcess { components: HashMap::new() }
    }
}

impl Process for HungerProcess {
    fn process(&self) -> Vec<Event> {
        println!("hungering!");
        vec![]
    }
}
