
use piston::input::generic_event::GenericEvent;

use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

use cascadecs::brain_process::BrainProcess;
use cascadecs::hunger_process::HungerProcess;
use cascadecs::fov_process::FovProcess;

pub struct Processes {
    processes: Vec<Box<Process>>,
    brain: Box<BrainProcess>,
}

impl Processes {
    pub fn new() -> Processes {
        let mut processes: Vec<Box<Process>> = vec![];
        processes.push(Box::new(HungerProcess::new()));
        processes.push(Box::new(FovProcess::new()));
        Processes {
            processes: processes,
            brain: Box::new(BrainProcess::new()),
        }
    }

    pub fn update_brain<E: GenericEvent>(&self, e: &E, components: &Components) -> Vec<Event> {
        self.brain.process_brain(e, components)
    }

    pub fn update(&self, components: &Components) -> Vec<Event> {
        self.processes.iter().flat_map(|process|
            process.process(components)
        ).collect()
    }
}
