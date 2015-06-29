use pixset::{
    Pix,
    Pixset,
};

use paths::{
    Path,
    Paths,
};

use renderable::{
    Vertex,
    Renderable,
};

use config::SQUARE_SIZE;

use loc::Loc;
use updatable::Updatable;
use utils::get_closest;
use world::World;

#[derive(Debug)]
pub struct Dood {
    loc: Loc,
    scale: f32,
    color: [f32; 3],
    pix: Pix,
    path: Path,
    hunger: f32,
}

impl Dood {
    pub fn new(loc: Loc) -> Dood {
        Dood {
            loc: loc,
            scale: SQUARE_SIZE as f32,
            hunger: 55.0,
            color: [250.0, 0.0, 0.0],
            pix: Pix::Dood,
            path: Vec::new(),
        }
    }
}

impl Paths for Dood {}

impl Updatable for Dood {
    fn update(&mut self, world: &World) {
        self.hunger -= 1.0;

        if self.path.is_empty() && self.hunger < 50.0 {
            if let Some(food_loc) = get_closest(self.loc, world.foods.keys().collect::<Vec<_>>()) {
                if food_loc == self.loc {
                    if let Some(food) = world.foods.get(&food_loc) {
                        self.hunger += food.borrow_mut().eat(20.0);
                    }
                } else {
                    self.path = self.path(&world.grid, self.loc, food_loc);
                }
            }
        }

        match self.path.pop() {
            Some(loc) => self.loc = loc,
            None => {},
        }
    }
}

impl Renderable for Dood {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (SQUARE_SIZE / 2) as f32;
        let x = (self.loc.x * SQUARE_SIZE) as f32 + offset;
        let y = (self.loc.y * SQUARE_SIZE) as f32 + offset;

        vec![
            // bottom left
            Vertex {
                vertex_position: [-0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[0],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },
            // bottom right
            Vertex {
                vertex_position: [0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[1],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },
            // top right
            Vertex {
                vertex_position: [0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[2],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },
            // top left
            Vertex {
                vertex_position: [-0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[3],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },
        ]
    }
}
