
use cascadecs::entity::Entity;
use cascadecs::components::Components;
use cascadecs::position_component::PositionComponent;

use super::flags::{Flags, HAS_ENTITY, IN_FOV, TRANSPARENT, PASSABLE};
use super::vertex::Vertex;

use chunk_loc::ChunkLoc;
use loc::Loc;
use size::Size;
use indices::Indices;
use terrain::Terrain;
use chunks::Chunks;
use world_coord::WorldCoord;
use pixset::Pixset;

// better Matrix set-up
// http://www.reddit.com/r/rust/comments/3exten/typechecked_matrix_operations_in_rust/ctk0pb4
// apparently you can do [u8, ..N]
// and associated consts:
// https://www.reddit.com/r/rust/comments/1z3dnx/ints_in_generic_type_parameters/
//
//
// use array vec or small vec
// https://github.com/bluss/arrayvec
// https://crates.io/crates/smallvec
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
            // TODO Arrays on nightlies implement Default
            terrain: vec![Terrain::None; len],
            flags: vec![TRANSPARENT | PASSABLE; len],
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

    // TODO I don't really like this
    pub fn get_flags(&mut self) -> &mut Vec<Flags> {
        &mut self.flags
    }

    pub fn insert_entity(&mut self, entity: Entity, components: &Components) {
        if let Some(pc) = components.get_position_component(entity) {
            self.entities.push(entity);

            let offset = match self.loc_to_indices(pc.loc) {
                None => return,
                Some(indices) => indices.to_1d(),
            };

            self.flags[offset].insert(HAS_ENTITY);
            if let Some(_) = components.get_opaque_component(entity) {
                self.flags[offset].remove(TRANSPARENT);
            }
            if let Some(_) = components.get_impassable_component(entity) {
                self.flags[offset].remove(PASSABLE);
            }
        }
    }

    pub fn clear_fov(&mut self) {
        for flag in self.flags.iter_mut() {
            flag.remove(IN_FOV);
        }
    }

    pub fn inflate(mut self, chunks: &mut Chunks, components: &Components) -> Scratch {
        let (tl_loc, br_loc) = self.to_loc_box();
        //Loc { x: -80, y: 80 }, Loc { x: 111, y: -111 }
        let size = Size { width: 16, height: 16 }; // TODO gross
        let tl_wc = WorldCoord::from_loc(&tl_loc).get_chunk_loc();
        let br_wc = WorldCoord::from_loc(&br_loc).get_chunk_loc();

        let width = size.width as usize;
        let mut offset_start: usize = 0;
        let mut offset_end: usize = width;

        for y in (br_wc.y..tl_wc.y + 1).rev() {
            for row in 0..size.height {
                for x in tl_wc.x..br_wc.x + 1 {
                    let chunk = chunks.get_chunk_mut(&ChunkLoc { x: x, y: y });
                    let start = (row * size.width) as usize;
                    let end = ((row + 1) * size.width) as usize;
                    let source = &chunk.get_terrain()[start..end];
                    self.terrain[offset_start..offset_end].clone_from_slice(source);
                    offset_start = offset_end;
                    offset_end = offset_start + width;
                }
            }
        }

        for y in (br_loc.y..tl_loc.y + 1).rev() {
            for x in (tl_loc.x..br_loc.x + 1) {
                // TODO dont like denormalized hash map
                // maybe make a chunk component and entities belong to those?
                let loc = Loc { x: x, y: y };
                if let Some(entity) = components.get_position_component_by_value(loc) {
                    self.insert_entity(*entity, components);
                }
            }
        }

        self
    }

    fn to_loc_box(&self) -> (Loc, Loc) {
        (Loc { x: self.loc.x, y: self.loc.y },
         Loc { x: self.loc.x + self.size.width - 1, y: self.loc.y - self.size.height + 1 })
    }

    pub fn loc_to_indices(&self, loc: Loc) -> Option<Indices> {
        let (tl, br) = self.to_loc_box();

        if loc.x >= tl.x || loc.y <= tl.y || loc.x <= br.x || loc.y >= br.y {
            let y_offset = self.loc.y - loc.y;
            let x_offset = loc.x - self.loc.x;
            Some(Indices { row: y_offset, col: x_offset, width: self.size.width })
        } else {
            None
        }

    }

    // TODO return &[Vertex] using vec as_slice?
    pub fn render(&self,
                  camera_loc: Loc,
                  camera_dim: Size,
                  tiles: &Pixset,
                  components: &Components)
                  -> (Vec<Vertex>, &[u32]) {
        // TODO get rid of the assumption on camera dim ratios, i.e. `* 2`
        let mut vertex_data: Vec<Vertex> = Vec::with_capacity(self.terrain.len() * 4 +
                                                              self.entities.len() * 4);

        let start = ((self.loc.y - camera_loc.y) *
                     self.size.width + camera_loc.x - self.loc.x) as usize;
        let end_camera_row = (camera_dim.height * 2) as usize;

        for (camera_row, row_terrain) in
            self.terrain[start..].chunks(camera_dim.width as usize).enumerate() {
            if camera_row % 2 == 1 {
                continue;
            };
            if camera_row == end_camera_row {
                break;
            };
            let row = camera_row / 2;
            for (col, terrain) in row_terrain.iter().enumerate() {
                let offset = camera_row * camera_dim.width as usize + col + start;
                if self.flags[offset].contains(IN_FOV) {
                    let loc = Loc { x: camera_loc.x + col as i32, y: camera_loc.y - row as i32 };
                    terrain.render(loc, &mut vertex_data, tiles);
                }
            }
        }

        for entity in self.entities.iter() {
            if let Some(&PositionComponent { loc }) = components.get_position_component(*entity) {
                // TODO maybe implement loc.contains(loc) and loc.outside(loc)
                if loc.x < camera_loc.x || loc.x > camera_loc.x + camera_dim.width ||
                   loc.y > camera_loc.y ||
                   loc.y < camera_loc.y - camera_dim.height {
                    continue;
                };
                if let Some(rc) = components.get_render_component(*entity) {
                    let offset = (((camera_loc.y - loc.y) * camera_dim.width * 2) +
                                  (loc.x - camera_loc.x)) as usize +
                                 start;
                    // so with it in an if it renders none and crashes
                    if self.flags[offset].contains(IN_FOV) {
                        rc.render(loc, &mut vertex_data, tiles);
                    }
                }
            }
        }

        let len = vertex_data.len();
        (vertex_data, &self.indices[..len * 4])
    }

    pub fn neighbors(&self, loc: Loc) -> Vec<Loc> {
        let Loc { x, y } = loc;

        let results: Vec<Loc> = vec![
            Loc { x: x + 1, y: y     },
            Loc { x: x,     y: y - 1 },
            Loc { x: x - 1, y: y     },
            Loc { x: x,     y: y + 1 },
        ];
        //if (x + y) % 2 == 0 { results.reverse(); }
        results
            .into_iter()
            .filter(|l| self.passable(l))
            .collect()
    }

    fn passable(&self, loc: &Loc) -> bool {
        match self.loc_to_indices(*loc) {
            None => false,
            Some(indices) => self.flags[indices.to_1d()].contains(PASSABLE),
        }
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
