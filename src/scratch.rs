
use cascadecs::entity::Entity;
use cascadecs::components::Components;
use cascadecs::position_component::PositionComponent;
use cascadecs::render_component::RenderComponent;

use config;

use chunk_loc::ChunkLoc;
use loc::Loc;
use size::Size;
use indices::Indices;
use terrain::Terrain;
use renderable::Vertex;
use chunks::Chunks;
use world_coord::WorldCoord;
use pixset::{Pix, Pixset};

//derives Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord and Hash
bitflags! {
    // attr can be added
    // x: i32,
    flags Flags: u32 {
        const NONE            = 0b00000000,
        const HAS_ENTITY      = 0b00000001,
        const HAS_ITEM        = 0b00000010,
        const BLOCKS_SIGHT    = 0b00000100,
        const BLOCKS_MOVEMENT = 0b00001000,
    }
}

// better Matrix set-up
// http://www.reddit.com/r/rust/comments/3exten/typechecked_matrix_operations_in_rust/ctk0pb4
pub struct Scratch {
    loc: Loc,
    size: Size,
    terrain: Vec<Terrain>,
    flags: Vec<Flags>,
    // vertices: Vec<Vertex>,
    entities: Vec<Entity>,
    indices: Vec<u32>,
}

impl Scratch {
    pub fn new(loc: Loc, size: Size) -> Scratch {
        let len = (size.width * size.height) as usize;
        // scratch is 36864 (192 x 192)
        // camera  is 96 x 64 (x: -32, y: 16)
        Scratch {
            loc: loc,
            size: size,
            terrain: vec![Terrain::None; len],
            flags: vec![NONE; len],
            // vertices: vec![NONE; len * 4],
            entities: vec![],
            indices: indices(len * 4),
        }
    }

    pub fn get_loc(&self) -> Loc {
        self.loc
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn inflate(mut self, chunks: &mut Chunks) -> Scratch {
        let loc_box = self.to_loc_box();
        let size = Size { width: 16, height: 16 }; // TODO gross
        let tl = WorldCoord::from_loc(&self.loc).get_chunk_loc();
        let br = WorldCoord::from_loc(&loc_box.1).get_chunk_loc();

        let width = size.width as usize;
        let mut offset_start: usize = 0;
        let mut offset_end: usize = width;

        for y in (br.y..tl.y + 1).rev() {
            for row in 0..size.height {
                for x in tl.x..br.x + 1 {
                    let chunk = chunks.get_chunk(&ChunkLoc { x: x, y: y });
                    let start = (row * size.width) as usize;
                    let end = ((row + 1) * size.width) as usize;
                    let source = &chunk.get_terrain()[start..end];
                    self.terrain[offset_start..offset_end].clone_from_slice(source);
                    offset_start = offset_end;
                    offset_end = offset_start + width;
                }
            }
        }

        // TODO obv less than ideal
        for y in (br.y..tl.y + 1).rev() {
            for x in tl.x..br.x + 1 {
                let chunk = chunks.get_chunk(&ChunkLoc { x: x, y: y });
                for entity in chunk.get_entities() {
                    self.entities.push(*entity);
                }
            }
        }

        self
    }

    pub fn insert_into_entities(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn to_loc_box(&self) -> (Loc, Loc) {
        (Loc { x: self.loc.x, y: self.loc.y },
         Loc { x: self.loc.x + self.size.width - 1, y: self.loc.y - self.size.height + 1 })
    }

    //struct WC(i32, i32);

    //impl Iterator for WC {
    //    type Item = (i32, i32);
    //
    //    fn next(&mut self) -> Option<(i32, i32)> {
    //        self.0 -= 1;
    //        self.1 -= 1;

    //        if self.0 == 0 {
    //            None
    //        } else {
    //            Some((self.0, self.1))
    //        }
    //    }
    //}

    //fn main() {
    //    for t in WC(4, 4) {
    //        println!("{:?}", t);
    //    }
    //}

    fn loc_to_indices(&self, loc: Loc) -> Indices {
        let (tl, br) = self.to_loc_box();
        assert!(loc.x >= tl.x);
        assert!(loc.y <= tl.y);
        assert!(loc.x <= br.x);
        assert!(loc.y >= br.y);
        let y_offset = self.loc.y - loc.y;
        let x_offset = loc.x - self.loc.x;
        Indices { row: y_offset, col: x_offset, width: self.size.width }
    }

    // TODO return &[Vertex] using vec as_slice?
    pub fn render(&self,
                  camera_loc: Loc,
                  camera_dim: Size,
                  tiles: &Pixset,
                  components: &Components)
                  -> (Vec<Vertex>, &Vec<u32>) {
        let mut vertex_data: Vec<Vertex> = Vec::with_capacity(self.terrain.len() * 4);

        let start = ((self.loc.y - camera_loc.y) *
                     self.size.width + camera_loc.x - self.loc.x) as usize;
        let end_camera_row = (camera_dim.height * 2) as usize;

        for (camera_row, row_terrain) in self.terrain[start..]
            .chunks(camera_dim.width as usize).enumerate() {
            if camera_row % 2 == 1 {
                continue;
            };
            if camera_row == end_camera_row {
                break;
            };
            let row = camera_row / 2;
            for (col, terrain) in row_terrain.iter().enumerate() {
                let x = ((camera_loc.x + col as i32) * config::SQUARE_SIZE) as f32;
                let y = ((camera_loc.y - row as i32) * config::SQUARE_SIZE) as f32;
                match terrain {
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
                    },
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
                    },
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
                    },
                    _ => {},
                }
            }
        }

        for entity in self.entities.iter() {
            if let Some(&PositionComponent { loc }) = components
                .get_position_component(*entity) {
                // TODO maybe implement loc.contains(loc) and loc.outside(loc)
                if loc.x < camera_loc.x || loc.x > camera_loc.x + camera_dim.width ||
                   loc.y > camera_loc.y || loc.y < camera_loc.y - camera_dim.height {
                    continue;
                };
                if let Some(&RenderComponent { ref pix, color }) = components
                    .get_render_component(*entity) {
                    let x = (loc.x * config::SQUARE_SIZE) as f32;
                    let y = (loc.y * config::SQUARE_SIZE) as f32;
                    vertex_data.push(Vertex {
                        vertex_position: [0.0, 0.0],
                        tex_coords: tiles.get(&pix)[0],
                        loc: [x, y],
                        scale: 16.0,
                        color: color,
                    });
                    vertex_data.push(Vertex {
                        vertex_position: [1.0, 0.0],
                        tex_coords: tiles.get(&pix)[1],
                        loc: [x, y],
                        scale: 16.0,
                        color: color,
                    });
                    vertex_data.push(Vertex {
                        vertex_position: [1.0, 1.0],
                        tex_coords: tiles.get(&pix)[2],
                        loc: [x, y],
                        scale: 16.0,
                        color: color,
                    });
                    vertex_data.push(Vertex {
                        vertex_position: [0.0, 1.0],
                        tex_coords: tiles.get(&pix)[3],
                        loc: [x, y],
                        scale: 16.0,
                        color: color,
                    });
                }
            }
        }

        (vertex_data, &self.indices)
    }
}

fn indices(length: usize) -> Vec<u32> {
    (0..(length / 4)).into_iter().flat_map(|i|
        vec![0, 1, 2, 0, 2, 3].into_iter().map(|j| (j + i * 4) as u32).collect::<Vec<u32>>()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::{indices, Scratch};
    use config;
    use loc::Loc;
    use size::Size;
    use pixset::Pixset;
    use indices::Indices;
    use chunk_loc::ChunkLoc;
    use world_coord::WorldCoord;

    //#[test]
    //// TODO finish me
    //fn it_returns_() {
    //    Scratch::new(Loc { x: -80, y: 80 }, Size { width: 192, height: 192 })
    //        .render(Loc { x: -50, y: 50 }, Size { width: 96, height: 64 }, &Pixset::new(16));
    //    assert!(true == false);
    //}

    #[test]
    fn loc_to_indices_it_returns_for_zero_zero() {
        let scratch = Scratch::new(Loc { x: 0, y: 0 }, Size { width: 192, height: 192 });
        let indices = Indices { row: 0, col: 0, width: 192 };
        assert!(scratch.loc_to_indices(Loc { x: 0, y: 0 }) == indices);
    }

    #[test]
    fn loc_to_indices_it_returns_for_zero_one() {
        let scratch = Scratch::new(Loc { x: 0, y: 0 }, Size { width: 192, height: 192 });
        let indices = Indices { row: 0, col: 1, width: 192 };
        assert!(scratch.loc_to_indices(Loc { x: 1, y: 0 }) == indices);
    }

    #[test]
    fn loc_to_indices_it_returns_for_one_one() {
        let scratch = Scratch::new(Loc { x: 0, y: 0 }, Size { width: 192, height: 192 });
        let indices = Indices { row: 1, col: 0, width: 192 };
        assert!(scratch.loc_to_indices(Loc { x: 0, y: -1 }) == indices);
    }

    #[test]
    fn loc_to_indices_it_returns_for_two_two() {
        let scratch = Scratch::new(Loc { x: -1, y: 1 }, Size { width: 192, height: 192 });
        let indices = Indices { row: 3, col: 3, width: 192 };
        assert!(scratch.loc_to_indices(Loc { x: 2, y: -2 }) == indices);
    }

    #[test]
    fn indices_it_returns_for_len_four() {
        assert_eq!(indices(4), [0u32, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn indices_it_returns_for_len_eight() {
        assert_eq!(indices(8), [0u32, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }
}
