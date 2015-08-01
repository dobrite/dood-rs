

use brain::Brain;

use cascadecs::event::Event;
use cascadecs::components::Components;

pub struct BrainComponent {
    pub brain: Brain,
}

impl BrainComponent {
    pub fn new(brain: Brain) -> Self {
        BrainComponent { brain: brain }
    }

    pub fn update(&self, components: &Components) -> Event {
        self.brain.update(components)
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
