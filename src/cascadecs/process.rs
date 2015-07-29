
use cascadecs::event::Event;
use cascadecs::components::Components;

pub trait Process {
    fn process(&self, components: &Components) -> Vec<Event>;
}
