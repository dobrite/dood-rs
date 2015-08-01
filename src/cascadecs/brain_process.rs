
use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::components::Components;
use cascadecs::brain_component::BrainComponent;

use dir::Dir;

pub struct BrainProcess;

impl BrainProcess {
    pub fn new() -> Self {
        BrainProcess
    }
}

impl Process for BrainProcess {
    fn process(&self, components: &Components) -> Vec<Event> {
        components
            .brain_components
            .iter()
            .map(|(&entity, brain_component)| brain_component.update(components))
            .filter(|event| *event != Event::None)
            .collect()
    }
}
