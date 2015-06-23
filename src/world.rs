use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use config::SQUARE_SIZE;

use dood::Dood;
use food::Food;
use grid::Grid;
use loc::Loc;
use loc_map::LocMap;
use renderable::Renderable;
use updatable::Updatable;
use window_loc::WindowLoc;

pub struct World {
    pub renderables: LocMap<Renderable>,
    pub updatables:  LocMap<Updatable>,
    pub foods: LocMap<Food>,
    pub doods: LocMap<Dood>,
    pub grid: Grid, // TODO prob doesn't need to be in World
    blocked: Vec<Loc>,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut renderables = HashMap::new();
        let mut updatables  = HashMap::new();
        let mut doods = HashMap::new();
        let mut foods = HashMap::new();

        // x, y
        let food_loc = (1, 1);
        let dood_loc = (-1, -1);

        let food = Rc::new(RefCell::new(Food::new(food_loc)));
        let dood = Rc::new(RefCell::new(Dood::new(dood_loc)));

        renderables.insert(food_loc, food.clone() as Rc<RefCell<Renderable>>);
        renderables.insert(dood_loc, dood.clone() as Rc<RefCell<Renderable>>);
        updatables.insert(dood_loc, dood.clone() as Rc<RefCell<Updatable>>);
        foods.insert(food_loc, food.clone());
        doods.insert(dood_loc, dood.clone());

        return World {
            renderables: renderables,
            updatables: updatables,
            foods: foods,
            doods: doods,
            grid: Grid::new(width, height),
            blocked: vec![], // TODO get rid of this
        }
    }

    pub fn spawn(&mut self, loc: Loc) {
        let food = Rc::new(RefCell::new(Food::new(loc)));
        self.renderables.insert(loc, food as Rc<RefCell<Renderable>>);
    }

    pub fn update(&self) {
        for (_, entity) in self.updatables.iter() {
            entity.borrow_mut().update(self);
        }
    }
}
