
use std::collections::HashMap;

use cascadecs::event::Event;
use cascadecs::entity::Entity;
use cascadecs::process::Process;
use cascadecs::render_component::RenderComponent;

pub struct RenderProcess {
    components: HashMap<Entity, RenderComponent>,
}

impl RenderProcess {
    pub fn new() -> RenderProcess {
        RenderProcess { components: HashMap::new() }
    }
}

impl Process for RenderProcess {
    fn process(&self) -> Vec<Event> {
        println!("rendering!");
        vec![]
    }
}
