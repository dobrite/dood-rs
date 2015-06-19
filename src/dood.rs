use pixset::{
    Pix,
    Pixset,
};

use paths::{
    Path,
    Paths,
};

use config::SQUARE_SIZE;
use entity::Entity;
use grid::Grid;
use loc::Loc;
use renderable::Vertex;

#[derive(Debug)]
pub struct Dood {
    loc: Loc,
    scale: f32,
    color: [f32; 3],
    hunger: f32,
    pix: Pix,
    path: Path,
}

impl Dood {
    pub fn new(loc: Loc, square_size: f32) -> Dood {
        return Dood {
            loc: loc,
            scale: square_size,
            hunger: 100.0,
            color: [0.2; 3],
            pix: Pix::Dood,
            path: Vec::new(),
        }
    }
}

impl Paths for Dood {}

impl Entity for Dood {
    fn update(&mut self, grid: &Grid, entities: &Vec<Loc>) {
        self.hunger -= 0.01;

        // TODO use hunger
        if self.path.is_empty() {
            self.path = self.path(&grid, &entities, self.loc, (10, 10));
        }

        match self.path.pop() {
            Some(loc) => self.loc = loc,
            None => {},
        }
    }

    fn render(&self, tiles: &Pixset) -> Vec<Vertex> {
        let offset = (SQUARE_SIZE / 2) as f32;
        let x = (self.loc.0 * SQUARE_SIZE) as f32 + offset;
        let y = (self.loc.1 * SQUARE_SIZE) as f32 + offset;

        return vec![
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
