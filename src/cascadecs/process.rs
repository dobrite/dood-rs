
use piston::input::GenericEvent;

use cascadecs::event::Event;
use cascadecs::components::Components;

pub trait Process {
    fn process<E: GenericEvent>(&self, e: &E, components: &Components) -> Vec<Event>;
}
