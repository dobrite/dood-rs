use pixset::{
    Pix,
    Pixset,
};

use renderable::{
    Vertex,
    Renderable,
};

use config::SQUARE_SIZE;

use loc::Loc;
use updatable::Updatable;
use world::World;

#[derive(Debug)]
pub struct Wall {
    loc: Loc,
    scale: f32,
    color: [f32; 3],
    pix: Pix,
}

impl Wall {
    pub fn new(loc: Loc) -> Wall {
        Wall {
            loc: loc,
            scale: SQUARE_SIZE as f32,
            color: [1.0; 3],
            pix: Pix::Wall,
        }
    }
}

impl Updatable for Wall {
    fn update(&mut self, world: &World) {}
}

impl Renderable for Wall {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (SQUARE_SIZE / 2) as f32;
        let x = (self.loc.x * SQUARE_SIZE) as f32 + offset;
        let y = (self.loc.y * SQUARE_SIZE) as f32 + offset;

        vec![
            Vertex {
                vertex_position: [-0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[0],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },  // left bottom
            Vertex {
                vertex_position: [0.5, -0.5],
                tex_coords: tiles.get(&self.pix)[1],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            },  // right bottom
            Vertex {
                vertex_position: [0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[2],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            }, // right top
            Vertex {
                vertex_position: [-0.5, 0.5],
                tex_coords: tiles.get(&self.pix)[3],
                loc: [x, y],
                scale: self.scale,
                color: self.color
            }, // left  top
        ]
    }
}
