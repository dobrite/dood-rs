
use piston::input::generic_event::GenericEvent;

use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

use cascadecs::brain_process::BrainProcess;
use cascadecs::hunger_process::HungerProcess;
use cascadecs::fov_process::FovProcess;

pub struct Processes {
    hunger_process: HungerProcess,
    fov_process: FovProcess,
    brain_process: BrainProcess,
}

impl Processes {
    pub fn new() -> Processes {
        Processes {
            hunger_process: HungerProcess::new(),
            fov_process: FovProcess::new(),
            brain_process: BrainProcess::new(),
        }
    }

    pub fn update<E: GenericEvent>(&self, e: &E, components: &Components) -> Vec<Event> {
        // TODO obv room for improvement
        let mut brain_events = BrainProcess.process(e, components);
        let mut hunger_events = HungerProcess.process(e, components);
        let mut fov_events = FovProcess.process(e, components);
        brain_events.append(&mut hunger_events);
        brain_events.append(&mut fov_events);
        brain_events
    }
}
