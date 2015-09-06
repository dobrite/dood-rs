
use render::Vertex;

use config;

use loc::Loc;
use pixset::{Pix, Pixset};

#[derive(Clone, Debug)]
pub enum Terrain {
    None,
    Dirt,
    Grass,
}

// enum with explicit discriminator
//enum Color {
//    Red = 0xff0000,
//    Green = 0x00ff00,
//    Blue = 0x0000ff,
//}
//http://rustbyexample.com/custom_types/enum/c_like.html

impl Terrain {
    pub fn render(&self, loc: Loc, vertex_data: &mut Vec<Vertex>, tiles: &Pixset) {
        let x = (loc.x * config::SQUARE_SIZE) as f32;
        let y = (loc.y * config::SQUARE_SIZE) as f32;
        match self {
            &Terrain::Dirt => {
                // bottom left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 0.0],
                    tex_coords: tiles.get(&Pix::Period)[0],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.4784, 0.3215, 0.1882]
                });
                // bottom right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 0.0],
                    tex_coords: tiles.get(&Pix::Period)[1],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.4784, 0.3215, 0.1882]
                });
                // top right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 1.0],
                    tex_coords: tiles.get(&Pix::Period)[2],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.4784, 0.3215, 0.1882]
                });
                // top left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 1.0],
                    tex_coords: tiles.get(&Pix::Period)[3],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.4784, 0.3215, 0.1882]
                });
            }
            &Terrain::Grass => {
                // bottom left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 0.0],
                    tex_coords: tiles.get(&Pix::Period)[0],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 255.0, 0.0]
                });
                // bottom right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 0.0],
                    tex_coords: tiles.get(&Pix::Period)[1],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 255.0, 0.0]
                });
                // top right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 1.0],
                    tex_coords: tiles.get(&Pix::Period)[2],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 255.0, 0.0]
                });
                // top left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 1.0],
                    tex_coords: tiles.get(&Pix::Period)[3],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 255.0, 0.0]
                });
            }
            &Terrain::None => {
                // bottom left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 0.0],
                    tex_coords: tiles.get(&Pix::Empty)[0],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 0.0, 0.0]
                });
                // bottom right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 0.0],
                    tex_coords: tiles.get(&Pix::Empty)[1],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 0.0, 0.0]
                });
                // top right
                vertex_data.push(Vertex {
                    vertex_position: [1.0, 1.0],
                    tex_coords: tiles.get(&Pix::Empty)[2],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 0.0, 0.0]
                });
                // top left
                vertex_data.push(Vertex {
                    vertex_position: [0.0, 1.0],
                    tex_coords: tiles.get(&Pix::Empty)[3],
                    loc: [x, y],
                    scale: 16.0,
                    color: [0.0, 0.0, 0.0]
                });
            }
        }
    }
}
