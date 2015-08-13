
use piston::input::GenericEvent;

use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::components::Components;
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
    fn process(&self, components: &Components) -> Vec<Event> {
        components.hunger_components.iter().map(|(&entity, ref hunger)| {
            Event::Hunger { entity: entity, minus_hunger: hunger.rate }
        }).collect()
    }
}
