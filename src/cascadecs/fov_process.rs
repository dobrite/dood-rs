
use cascadecs::event::Event;
use cascadecs::process::Process;
use cascadecs::components::Components;

pub struct FovProcess;

impl FovProcess {
    pub fn new() -> Self {
        FovProcess
    }
}

impl Process for FovProcess {
    // TODO can't we do more in here rather than in apply? i.e. gather everything that's needed
    // rather than looking it up during apply. This portion (hopefully) can be threaded.
    fn process(&self, components: &Components) -> Vec<Event> {
        components.fov_components.keys().map(|&entity| {
            Event::ComputeFov { entity: entity }
        }).collect()
    }
}
