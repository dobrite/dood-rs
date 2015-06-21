use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use config::SQUARE_SIZE;

use dood::Dood;
use food::Food;
use grid::Grid;
use loc::Loc;
use renderable::Renderable;
use updatable::Updatable;
use window_loc::WindowLoc;

pub struct World {
    pub renderables: HashMap<Loc, Rc<RefCell<Renderable>>>,
    pub updatables:  HashMap<Loc, Rc<RefCell<Updatable>>>,
    grid: Grid,
    blocked: Vec<Loc>,
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut renderables = HashMap::new();
        let mut updatables  = HashMap::new();

        // x, y
        let food_loc = (1, 1);
        let dood_loc = (-1, -1);

        let food = Rc::new(RefCell::new(Food::new(food_loc, SQUARE_SIZE as f32)));
        let dood = Rc::new(RefCell::new(Dood::new(dood_loc, SQUARE_SIZE as f32)));

        renderables.insert(food_loc, food.clone() as Rc<RefCell<Renderable>>);
        renderables.insert(dood_loc, dood.clone() as Rc<RefCell<Renderable>>);
        updatables.insert(dood_loc, dood.clone() as Rc<RefCell<Updatable>>);

        return World {
            renderables: renderables,
            updatables: updatables,
            grid: Grid::new(width, height),
            blocked: vec![], // TODO get rid of this
        }
    }

    pub fn spawn(&mut self, loc: Loc) {
        self.renderables.insert(loc, Rc::new(RefCell::new(Food::new(loc, SQUARE_SIZE as f32))) as Rc<RefCell<Renderable>>);
    }

    pub fn update(&mut self) {
        for (_, entity) in self.updatables.iter_mut() {
            entity.borrow_mut().update(&self.grid, &self.blocked);
        }
    }
}
