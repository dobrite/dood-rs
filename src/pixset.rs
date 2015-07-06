use std::collections::HashMap;

pub type TexCoords = [[f32; 2]; 4];

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Pix {
    Dood,
    Food,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
    Wall,
    Empty,
}

pub struct Pixset {
    pub tiles: HashMap<Pix, TexCoords>,
    pub total_tiles: i32,
}

impl Pixset {
    pub fn new(total_tiles: i32) -> Pixset {
        // TODO fix
        //let tiles = vec![
        //    Pix::DownArrow, Pix::LeftArrow,  Pix::Dood, Pix::Food,
        //    Pix::UpArrow,   Pix::RightArrow, Pix::None, Pix::None,
        //    Pix::None,      Pix::None,       Pix::None, Pix::None,
        //    Pix::None,      Pix::None,       Pix::None, Pix::None,
        //];

        //let tile_dim: i32 = (total_tiles as f32).sqrt() as i32;

        //for y in (0..tile_dim) {
        //    for x in (0..tile_dim) {
        //    }
        //}

        let mut tiles: HashMap<Pix, TexCoords> = HashMap::new();

        // TODO fix
        tiles.insert(Pix::DownArrow,  get_tex_coords(total_tiles, vec![0, 3]));
        tiles.insert(Pix::LeftArrow,  get_tex_coords(total_tiles, vec![1, 3]));
        tiles.insert(Pix::Dood,       get_tex_coords(total_tiles, vec![2, 3]));
        tiles.insert(Pix::Food,       get_tex_coords(total_tiles, vec![3, 3]));
        tiles.insert(Pix::UpArrow,    get_tex_coords(total_tiles, vec![0, 2]));
        tiles.insert(Pix::RightArrow, get_tex_coords(total_tiles, vec![1, 2]));
        tiles.insert(Pix::Wall,       get_tex_coords(total_tiles, vec![2, 2]));
        tiles.insert(Pix::Empty,      get_tex_coords(total_tiles, vec![3, 2]));

        Pixset {
            tiles: tiles,
            total_tiles: total_tiles,
        }
    }

    pub fn get(&self, pix: &Pix) -> [[f32; 2]; 4] {
        *self.tiles.get(pix).unwrap()
    }
}

fn get_tex_coords(total_tiles: i32, loc: Vec<i32>) -> [[f32; 2]; 4] {
    let tile_dim: f32 = (total_tiles as f32).sqrt();
    let per_tile: f32 = 1.0 / tile_dim;

    let top = 1.0 - loc[1] as f32 * per_tile;
    let right = (loc[0] + 1) as f32 * per_tile;
    let bottom = 1.0 - (loc[1] + 1) as f32 * per_tile;
    let left = loc[0] as f32 * per_tile;

    [
        [left,  top],
        [right, top],
        [right, bottom],
        [left,  bottom],
    ]
}

#[cfg(test)]
mod tests {
    use super::get_tex_coords;

    #[test]
    fn it_gets_tex_coords_for_top_left() {
        assert!(get_tex_coords(16, vec![0, 0]) == [
          [0.0,  1.0],
          [0.25, 1.0],
          [0.25, 0.75],
          [0.0,  0.75],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_top_right() {
        assert!(get_tex_coords(16, vec![3, 0]) == [
          [0.75, 1.0],
          [1.0,  1.0],
          [1.0,  0.75],
          [0.75, 0.75],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_bottom_right() {
        assert!(get_tex_coords(16, vec![3, 3]) == [
          [0.75, 0.25],
          [1.0,  0.25],
          [1.0,  0.0],
          [0.75, 0.0],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_bottom_left() {
        assert!(get_tex_coords(16, vec![0, 3]) == [
          [0.0,  0.25],
          [0.25, 0.25],
          [0.25, 0.0],
          [0.0,  0.0],
        ]);
    }
}
