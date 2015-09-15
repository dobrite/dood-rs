
use std::sync::mpsc;

use piston::input::GenericEvent;

use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

pub struct BrainProcess;

impl BrainProcess {
    pub fn new() -> Self {
        BrainProcess
    }
}

impl Process for BrainProcess {
    fn process<E: GenericEvent>(&self, e: &E, components: &Components) -> Vec<Event> {
        let (send, recv) = mpsc::channel();
        components.brain_iter().map(|(&entity, bc)|
            bc.update(e, entity, components, send.clone())
        ).collect::<Vec<_>>();
        drop(send);
        recv.iter().collect()
    }
}
