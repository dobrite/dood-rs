
use cascadecs::event::Event;

pub trait Process {
    fn process(&self) -> Vec<Event>;
}
