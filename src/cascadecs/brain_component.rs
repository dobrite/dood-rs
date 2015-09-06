
use piston::input;

use std::fmt;
use std::sync::mpsc;

use ai_behavior;

use cascadecs::entity::Entity;
use cascadecs::event::Event;
use cascadecs::components::Components;

use action::Action;
use brain::Brain;

pub struct BrainComponent {
    // TODO rename to type or something
    pub kind: Brain,
    pub target: Option<Entity>,
    pub state: ai_behavior::State<Action, ()>,
}

impl fmt::Debug for BrainComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl BrainComponent {
    pub fn new(brain: Brain) -> Self {
        BrainComponent { kind: brain, target: None, state: Brain::new_state(brain) }
    }

    pub fn update<E: input::GenericEvent>(&self,
                                          e: &E,
                                          entity: Entity,
                                          components: &Components,
                                          send: mpsc::Sender<Event>) {
        self.kind.update(e, entity, components, send)
    }
}
