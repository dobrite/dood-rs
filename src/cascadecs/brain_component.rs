
use piston::input;

use std::fmt;
use std::sync::mpsc;

use ai_behavior;

use cascadecs::entity;
use cascadecs::{event, components};

use action;
use brain;

pub struct BrainComponent {
    // TODO rename to type or something
    pub brain: brain::Brain,
    pub state: ai_behavior::State<action::Action, ()>,
}

impl fmt::Debug for BrainComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.brain)
    }
}

impl BrainComponent {
    pub fn new(brain: brain::Brain) -> Self {
        BrainComponent {
            brain: brain,
            state: brain::Brain::new_state(brain),
        }
    }

    pub fn update<E: input::GenericEvent>(&self, e: &E, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        self.brain.update(e, entity, components, send)
    }
}

//self.hunger -= 1.0;

//if self.path.is_empty() && self.hunger < 50.0 {
//    if let Some(food_loc) = get_closest(
//            self.loc, chunks.foods.keys().collect::<Vec<_>>()) {
//        if food_loc == self.loc {
//            if let Some(food) = chunks.foods.get(&food_loc) {
//                self.hunger += food.borrow_mut().eat(20.0);
//            }
//        } else {
//            self.path = self.path(&chunks.grid, self.loc, food_loc);
//        }
//    }
//}

//match self.path.pop() {
//    Some(loc) => self.loc = loc,
//    None => {},
//}
//
//pub fn eat(&mut self, usage: f32) -> f32 {
//    let ate = if self.noms - usage <= 0.0 {
//        self.noms
//    } else {
//        usage
//    };
//    self.noms -= usage;
//    ate
//}
//
//pub fn get_noms(&self) -> f32 {
//    self.noms
//}
