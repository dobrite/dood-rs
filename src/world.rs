use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{
    Rc,
    Weak,
};

use dood::Dood;
use food::Food;
use fov::Fov;
use grid::Grid;
use has_loc::HasLoc;
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
    pub fovs: Vec<Rc<RefCell<Fov>>>,
    pub grid: Grid, // TODO prob doesn't need to be in World
}

impl World {
    pub fn new(width: i32, height: i32) -> World {
        // 64, 48
        let mut renderables = HashMap::new();
        let mut updatables  = HashMap::new();
        let mut doods = HashMap::new();
        let mut foods = HashMap::new();
        let mut walls = HashMap::new();
        let mut fovs  = Vec::new();

        let food_loc = Loc { x: 22, y: 22 };
        let dood_loc = Loc { x: 20, y: 20 };

        let food = Rc::new(RefCell::new(Food::new(food_loc)));
        let dood = Rc::new(RefCell::new(Dood::new(dood_loc)));
        let fov  = Rc::new(RefCell::new(Fov::new((dood.clone() as Rc<RefCell<HasLoc>>).downgrade(), width, height)));

        renderables.insert(food_loc, food.clone() as Rc<RefCell<Renderable>>);
        renderables.insert(dood_loc, dood.clone() as Rc<RefCell<Renderable>>);
        //renderables.insert(dood_loc, fov.clone() as Rc<RefCell<Renderable>>);

        updatables.insert(food_loc, food.clone() as Rc<RefCell<Updatable>>);
        updatables.insert(dood_loc, dood.clone() as Rc<RefCell<Updatable>>);
        //updatables.insert(dood_loc, fov.clone() as Rc<RefCell<Updatable>>);

        foods.insert(food_loc, food.clone());
        doods.insert(dood_loc, dood.clone());
        fovs.push(fov);

        World {
            renderables: renderables,
            updatables: updatables,
            foods: foods,
            doods: doods,
            walls: walls,
            fovs: fovs,
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
        for fov in &self.fovs {
            fov.borrow_mut().set_transparent(loc.x, loc.y, false)
        }
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
