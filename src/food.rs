use pixset::{
    Pix,
    Pixset,
};

use renderable::{
    Vertex,
    Renderable,
};

use loc::Loc;
use updatable::Updatable;
use world::World;

#[derive(Debug)]
pub struct Food {
    loc: Loc,
    scale: f32,
    color: [f32; 3],
    pix: Pix,
    noms: f32,
}

impl Food {
    pub fn new(loc: Loc) -> Food {
        Food {
            loc: loc,
            scale: SQUARE_SIZE as f32,
            color: [0.2313725, 0.3254902, 0.1372549],
            pix: Pix::Food,
            noms: 100.0,
        }
    }

    pub fn eat(&mut self, usage: f32) -> f32 {
        let ate = if self.noms - usage <= 0.0 { self.noms } else { usage };
        self.noms -= usage;
        ate
    }

    pub fn get_noms(&self) -> f32 {
        self.noms
    }
}

impl Updatable for Food {
    fn update(&mut self, world: &World) {
        self.noms -= 1.0;
    }
}

impl Renderable for Food {
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
