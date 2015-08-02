
use std::sync::mpsc;

use cascadecs::{entity, event, components};

use dir;

#[derive(Clone, Debug)]
pub enum Brain {
    None,
    Dood,
}

impl Brain {
    pub fn update(&self, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        match *self {
            Brain::None => self.none(entity, send),
            Brain::Dood => self.dood(entity, send),
        }
    }

    fn none(&self, entity: entity::Entity, send: mpsc::Sender<event::Event>) {
        send.send(event::Event::None);
    }

    fn dood(&self, entity: entity::Entity, send: mpsc::Sender<event::Event>) {
        send.send(event::Event::Movement { entity: entity, dir: dir::Dir::Down });
    }
}
