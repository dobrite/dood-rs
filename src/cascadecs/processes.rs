
use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

use cascadecs::movement_process::MovementProcess;
use cascadecs::hunger_process::HungerProcess;

pub struct Processes {
    processes: Vec<Box<Process>>,
}

impl Processes {
    pub fn new() -> Processes {
        let mut processes: Vec<Box<Process>> = vec![];
        processes.push(Box::new(HungerProcess::new()));
        processes.push(Box::new(MovementProcess::new()));
        Processes { processes: processes }
    }

    pub fn update(&self, components: &Components) -> Vec<Event> {
        self.processes.iter().flat_map(|process|
            process.process(components)
        ).collect()
    }
}
