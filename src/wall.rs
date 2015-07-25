
use pixset::{Pix, Pixset};

use renderable::{Vertex, Renderable};

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
    pub fn new(loc: Loc, square_size: i32) -> Wall {
        Wall { loc: loc, scale: square_size as f32, color: [1.0; 3], pix: Pix::Wall }
    }
}

impl Updatable for Wall {
    fn update(&mut self, _: &World) {
    }
}

impl Renderable for Wall {
    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (self.scale / 2.0) as f32;
        let x = (self.loc.x as f32 * self.scale) + offset;
        let y = (self.loc.y as f32 * self.scale) + offset;

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
