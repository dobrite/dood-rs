
use std::sync::mpsc;

use piston::input::GenericEvent;

use cascadecs::event;
use cascadecs::process;
use cascadecs::components;

use dir::Dir;

pub struct BrainProcess;

impl BrainProcess {
    pub fn new() -> Self {
        BrainProcess
    }
}

impl BrainProcess {
    pub fn process_brain<E: GenericEvent>(&self, e: &E, comps: &components::Components) -> Vec<event::Event> {
        let (send, recv) = mpsc::channel();
        comps.brain_components.iter().map(|(&ent, bc)|
            bc.update(e, ent, comps, send.clone())
        ).collect::<Vec<_>>();
        drop(send);
        recv.iter().collect()
    }
}
