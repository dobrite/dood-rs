use config;
use camera::Camera;
use loc::Loc;
use size::Size;
use terrain::Terrain;
use renderable::Vertex;
use world_coord::WorldCoord;
use pixset::{
    Pix,
    Pixset,
};

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

pub struct Scratch {
    loc: Loc,
    width: i32, // TODO make Size
    height: i32, // TODO make Size
    terrain:  Vec<Terrain>,
    flags:    Vec<Flags>,
    //vertices: Vec<Vertex>,
    indices:  Vec<u32>,
}

impl Scratch {
    pub fn new(loc: Loc) -> Scratch {
        let width = config::SCRATCH_CHUNKS_WIDTH * config::CHUNK_WIDTH;
        let height = config::SCRATCH_CHUNKS_HEIGHT * config::CHUNK_HEIGHT;
        let len = (width * height) as usize;
        // scratch is 36864 (192 x 192)
        // camera  is 96 x 64 (x: -50, y: -50)

        Scratch {
            loc: loc,
            width: width,
            height: height,
            // TODO move back to None
            terrain: vec![Terrain::Dirt; len],
            flags: vec![NONE; len],
            //vertices: vec![NONE; len * 4],
            indices:  indices(len * 4),
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

    // TODO return &[Vertex] using vec as_slice?
    pub fn render(&self, camera_loc: Loc, camera_dim: Size, tiles: &Pixset) -> (Vec<Vertex>, &Vec<u32>) {
        let tile_offset = (config::SQUARE_SIZE / 2) as f32;

        let camera_offset_x = camera_loc.x - self.loc.x;
        let camera_offset_y = camera_loc.y - self.loc.y;
        let x_diff = (self.width - camera_dim.width) as usize;
        let y_diff = self.height - camera_dim.height;

        assert!(camera_offset_x >= 0);
        assert!(camera_offset_y <= 0);
        assert!(x_diff >= 0);
        assert!(y_diff >= 0);

        // Box<[Vertex; 256];
        let mut vertex_data: Vec<Vertex> = Vec::with_capacity(self.terrain.len() * 4); // TODO over allocation
        let mut start = (camera_offset_y * -1 * self.width + camera_offset_x) as usize;
        let mut end = 0;
        let camera_width  = camera_dim.width  as usize; // TODO consider passing in like this
        let camera_height = camera_dim.height as usize; // TODO consider passing in like this
        let end = (camera_height * 2) as i32;
        let tile_offset = (config::SQUARE_SIZE / 2) as f32;

        let mut i: i32 = -1;
        let x_offset = (camera_loc.x * config::SQUARE_SIZE) as f32;
        let y_offset = ((camera_dim.height - camera_loc.y - 1) * config::SQUARE_SIZE) as f32;
        for row_terrain in self.terrain[start..].chunks(camera_width) {
            i += 1;
            if i % 2 == 1 { continue };
            if i == end { break };
            let mut col: i32 = 0;
            let row: i32 = i / 2;
            for terrain in row_terrain {
                let x: f32 = (col * config::SQUARE_SIZE) as f32 + tile_offset + x_offset;
                let y: f32 = (row * config::SQUARE_SIZE) as f32 + tile_offset - y_offset;
                let vertices = match terrain {
                    &Terrain::Dirt => {
                        // bottom left
                        vertex_data.push(Vertex {
                            vertex_position: [-0.5, -0.5],
                            tex_coords: tiles.get(&Pix::Food)[0],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.4784, 0.3215, 0.1882]
                        });
                        // bottom right
                        vertex_data.push(Vertex {
                            vertex_position: [0.5, -0.5],
                            tex_coords: tiles.get(&Pix::Food)[1],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.4784, 0.3215, 0.1882]
                        });
                        // top right
                        vertex_data.push(Vertex {
                            vertex_position: [0.5, 0.5],
                            tex_coords: tiles.get(&Pix::Food)[2],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.4784, 0.3215, 0.1882]
                        });
                        // top left
                        vertex_data.push(Vertex {
                            vertex_position: [-0.5, 0.5],
                            tex_coords: tiles.get(&Pix::Food)[3],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.4784, 0.3215, 0.1882]
                        });
                    },
                    &Terrain::Grass => {
                        // bottom left
                        vertex_data.push(Vertex {
                            vertex_position: [-0.5, -0.5],
                            tex_coords: tiles.get(&Pix::Food)[0],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.0, 255.0, 0.0]
                        });
                        // bottom right
                        vertex_data.push(Vertex {
                            vertex_position: [0.5, -0.5],
                            tex_coords: tiles.get(&Pix::Food)[1],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.0, 255.0, 0.0]
                        });
                        // top right
                        vertex_data.push(Vertex {
                            vertex_position: [0.5, 0.5],
                            tex_coords: tiles.get(&Pix::Food)[2],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.0, 255.0, 0.0]
                        });
                        // top left
                        vertex_data.push(Vertex {
                            vertex_position: [-0.5, 0.5],
                            tex_coords: tiles.get(&Pix::Food)[3],
                            loc: [x, y],
                            scale: 16.0,
                            color: [0.0, 255.0, 0.0]
                        });
                    },
                    _ => {},
                };
                col += 1;
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
    use loc::Loc;
    use size::Size;
    use pixset::Pixset;
    use indices::Indices;
    use chunk_loc::ChunkLoc;
    use world_coord::WorldCoord;

    #[test]
    fn it_returns_() {
        Scratch::new(Loc { x: -80, y: 80 }).render(Loc { x: -50, y: 50 }, Size { width: 96, height: 64 }, &Pixset::new(16));
        assert!(true == false);
    }

    #[test]
    fn it_returns_indices_for_len_four() {
        assert!(indices(4) == [0u32, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn it_returns_indices_for_len_eight() {
        assert!(indices(8) == [0u32, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }
}
