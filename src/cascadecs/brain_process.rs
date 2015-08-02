
use std::sync::mpsc;

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

impl process::Process for BrainProcess {
    fn process(&self, comps: &components::Components) -> Vec<event::Event> {
        let (send, recv) = mpsc::channel();
        comps.brain_components.iter().map(|(&ent, bc)| bc.update(ent, comps, send.clone())).collect::<Vec<_>>();
        drop(send);
        recv.iter().collect()
    }
}
