
use piston::input::GenericEvent;

use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

pub struct HungerProcess;

impl HungerProcess {
    pub fn new() -> Self {
        HungerProcess
    }
}

impl Process for HungerProcess {
    fn process<E: GenericEvent>(&self, e: &E, components: &Components) -> Vec<Event> {
        components.hunger_iter().map(|(&entity, ref hunger)| {
            Event::Hunger { entity: entity, minus_hunger: hunger.rate }
        }).collect()
    }
}
