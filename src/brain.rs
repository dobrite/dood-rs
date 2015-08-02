

use std::sync::mpsc;

use ai_behavior;
use ai_behavior::{While, Action, WaitForever, WhenAny, Wait, Sequence};

use cascadecs::{entity, event, components};

use action::Action;
use dir;

#[derive(Clone, Copy, Debug)]
pub enum Brain {
    Dood,
}

impl Brain {
    pub fn new_state(brain: Brain) -> ai_behavior::State<Action, ()> {
        match brain {
            Brain::Dood => {
                let behavior = While(Box::new(WaitForever), vec![Action(Action::GoDown), Action(Action::ComeUp)]);
                ai_behavior::State::new(behavior)
            }
        }
    }

    pub fn update(&self, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        match *self {
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

