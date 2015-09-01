
use rand;

use cascadecs::entity::Entity;

use size::Size;
use terrain::Terrain;

pub struct Chunk {
    size: Size,
    terrain: Vec<Terrain>,
}

impl Chunk {
    pub fn new(size: Size) -> Chunk {
        let mut terrain = vec![Terrain::Dirt; (size.width * size.height) as usize];

        for terr in &mut terrain {
            if rand::random::<bool>() {
                *terr = Terrain::Grass;
            }
        }

        Chunk { size: size, terrain: terrain }
    }

    pub fn get_terrain(&self) -> &Vec<Terrain> {
        &self.terrain
    }
}
