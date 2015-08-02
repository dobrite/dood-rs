
use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::components::Components;
use cascadecs::position_component::PositionComponent;

use dir::Dir;

pub struct MovementProcess;

impl MovementProcess {
    pub fn new() -> Self {
        MovementProcess
    }
}

impl Process for MovementProcess {
    fn process(&self, components: &Components) -> Vec<Event> {
        //components.position_components.iter().map(|(&entity, _)| {
        //    if let Some(brain) = components.brain_components.get(&entity) {
        //        Event::Movement { entity: entity, dir: Dir::Down }
        //    } else {
        //        Event::None
        //    }
        //}).filter(|event| *event != Event::None).collect()
        vec![]
    }
}
