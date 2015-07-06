use config;
use terrain::Terrain;
use renderable::Vertex;
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
    terrain: Vec<Terrain>,
    flags: Vec<Flags>,
}

impl Scratch {
    pub fn new() -> Scratch {
        let len = (config::SCRATCH_CHUNKS_WIDTH *
            config::CHUNK_WIDTH *
            config::SCRATCH_CHUNKS_HEIGHT *
            config::CHUNK_HEIGHT) as usize;

        Scratch {
            // TODO move back to None
            terrain: vec![Terrain::Dirt; len],
            flags: vec![NONE; len],
        }
    }

    // TODO return &[Vertex] using vec as_slice?
    pub fn render(&self, tiles: &Pixset) -> (Vec<Vertex>, Vec<u32>) {
        let width  = config::SCRATCH_CHUNKS_WIDTH  * config::CHUNK_WIDTH;
        let height = config::SCRATCH_CHUNKS_HEIGHT * config::CHUNK_HEIGHT;
        let offset = (config::SQUARE_SIZE / 2) as f32;

        // Box<[[Vertex; 16]; 16];
        let mut vertex_data: Vec<Vertex> = Vec::new();

        let mut count = 0;
        for terrain in self.terrain.iter() {
            let square_x = count / width;
            let square_y = count % height;
            let x: f32 = (square_x * config::SQUARE_SIZE) as f32 + offset;
            let y: f32 = (square_y * config::SQUARE_SIZE) as f32 + offset;
            let vertices = match terrain {
                &Terrain::Dirt => vec![
                    // bottom left
                    Vertex {
                        vertex_position: [-0.5, -0.5],
                        tex_coords: tiles.get(&Pix::Food)[0],
                        loc: [x, y],
                        scale: 16.0,
                        color: [255.0, 0.0, 0.0]
                    },
                    // bottom right
                    Vertex {
                        vertex_position: [0.5, -0.5],
                        tex_coords: tiles.get(&Pix::Food)[1],
                        loc: [x, y],
                        scale: 16.0,
                        color: [255.0, 0.0, 0.0]
                    },
                    // top right
                    Vertex {
                        vertex_position: [0.5, 0.5],
                        tex_coords: tiles.get(&Pix::Food)[2],
                        loc: [x, y],
                        scale: 16.0,
                        color: [255.0, 0.0, 0.0]
                    },
                    // top left
                    Vertex {
                        vertex_position: [-0.5, 0.5],
                        tex_coords: tiles.get(&Pix::Food)[3],
                        loc: [x, y],
                        scale: 16.0,
                        color: [255.0, 0.0, 0.0]
                    },
                ],
                _ => vec![],
            };

            for vertex in vertices {
                vertex_data.push(vertex);
            }

            count += 1;
        }

        let len = vertex_data.len();
        (vertex_data, indices(len))
    }
}

pub fn indices(length: usize) -> Vec<u32> {
    (0..(length / 4)).into_iter().flat_map(|i|
        vec![0, 1, 2, 0, 2, 3].into_iter().map(|j| (j + i * 4) as u32).collect::<Vec<u32>>()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::indices;

    #[test]
    fn it_returns_indices_for_len_four() {
        assert!(indices(4) == [0u32, 1, 2, 0, 2, 3]);
    }

    #[test]
    fn it_returns_indices_for_len_eight() {
        assert!(indices(8) == [0u32, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]);
    }
}
