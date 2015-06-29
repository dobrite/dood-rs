use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dood::Dood;
use food::Food;
use grid::Grid;
use loc::Loc;
use loc_map::LocMap;
use renderable::Renderable;
use updatable::Updatable;
use wall::Wall;

pub struct World {
    pub renderables: LocMap<Renderable>,
    pub updatables:  LocMap<Updatable>,
    pub foods: LocMap<Food>,
    pub doods: LocMap<Dood>,
    pub walls: LocMap<Wall>,
    pub grid: Grid, // TODO prob doesn't need to be in World
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        let mut renderables = HashMap::new();
        let mut updatables  = HashMap::new();
        let mut doods = HashMap::new();
        let mut foods = HashMap::new();
        let mut walls = HashMap::new();

        // x, y
        let food_loc = Loc { x:  1, y:  1 };
        let dood_loc = Loc { x: -1, y: -1 };

        let food = Rc::new(RefCell::new(Food::new(food_loc)));
        let dood = Rc::new(RefCell::new(Dood::new(dood_loc)));

        renderables.insert(food_loc, food.clone() as Rc<RefCell<Renderable>>);
        renderables.insert(dood_loc, dood.clone() as Rc<RefCell<Renderable>>);
        updatables.insert(food_loc, food.clone() as Rc<RefCell<Updatable>>);
        updatables.insert(dood_loc, dood.clone() as Rc<RefCell<Updatable>>);
        foods.insert(food_loc, food.clone());
        doods.insert(dood_loc, dood.clone());

        World {
            renderables: renderables,
            updatables: updatables,
            foods: foods,
            doods: doods,
            walls: walls,
            grid: Grid::new(width, height),
        }
    }

    pub fn spawn_food(&mut self, loc: Loc) {
        let food = Rc::new(RefCell::new(Food::new(loc)));
        self.renderables.insert(loc, food.clone() as Rc<RefCell<Renderable>>);
        self.foods.insert(loc, food.clone());
    }

    pub fn spawn_wall(&mut self, loc: Loc) {
        let wall = Rc::new(RefCell::new(Wall::new(loc)));
        self.renderables.insert(loc, wall.clone() as Rc<RefCell<Renderable>>);
        self.walls.insert(loc, wall.clone());
    }

    pub fn update(&self) {
        for (_, entity) in self.updatables.iter() {
            entity.borrow_mut().update(self);
        }
    }

    pub fn vacuum(&mut self) {
        let mut remove = vec![];
        self.to_remove(&mut remove);
        for loc in remove.iter() {
            self.foods.remove(loc);
            self.renderables.remove(loc);
            self.updatables.remove(loc);
        }
    }

    fn to_remove(&self, remove: &mut Vec<Loc>) {
        for (loc, food) in self.foods.iter() {
            if food.borrow().get_noms() <= 0.0 {
                remove.push(*loc);
            }
        }
    }
}
