
use cascadecs::event::Event;
use cascadecs::components::Components;

#[derive(Clone, Debug)]
pub enum Brain {
    None,
    Dood,
}

impl Brain {
    pub fn update(&self, components: &Components) -> Event {
        match *self {
            Brain::None => println!("nothing"),
            Brain::Dood => println!("pathing!")
        }
        Event::None
    }
}
