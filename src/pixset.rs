use std::collections::HashMap;

use square::TexCoords;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Pix {
    None,
    Dood,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
}

pub struct Pixset {
    pub tiles: HashMap<Pix, TexCoords>
}

impl Pixset {
    pub fn new(total_tiles: i32, square_size: i32) -> Pixset {
        // TODO fix
        let tiles = vec![
            Pix::DownArrow, Pix::LeftArrow,  Pix::Dood, Pix::None,
            Pix::UpArrow,   Pix::RightArrow, Pix::None, Pix::None,
            Pix::None,      Pix::None,       Pix::None, Pix::None,
            Pix::None,      Pix::None,       Pix::None, Pix::None,
        ];

        let tile_dim: i32 = (total_tiles as f32).sqrt() as i32;

        //for y in (0..tile_dim) {
        //    for x in (0..tile_dim) {
        //    }
        //}

        let mut tiles: HashMap<Pix, TexCoords> = HashMap::new();

        // TODO fix
        tiles.insert(Pix::DownArrow,  get_tex_coords(vec![0, 0], total_tiles, square_size));
        tiles.insert(Pix::LeftArrow,  get_tex_coords(vec![1, 0], total_tiles, square_size));
        tiles.insert(Pix::Dood,       get_tex_coords(vec![2, 0], total_tiles, square_size));
        tiles.insert(Pix::None,       get_tex_coords(vec![3, 0], total_tiles, square_size));
        tiles.insert(Pix::UpArrow,    get_tex_coords(vec![0, 1], total_tiles, square_size));
        tiles.insert(Pix::RightArrow, get_tex_coords(vec![1, 1], total_tiles, square_size));

        return Pixset {
            tiles: tiles,
        }
    }

    pub fn get(&self, pix: &Pix) -> [[f32; 2]; 4] {
        return *self.tiles.get(pix).unwrap()
    }
}

// 0.0 (x),   0.0 (y)

// 0.0 left,  1.0 top
// 1.0 right, 1.0 top
// 1.0 right, 0.0 bottom
// 0.0 left,  0.0 bottom

// 0,0, 1,0, 2,0, 3,0
// 0,1, 1,1, 2,1, 3,1
// 0,2, 1,2, 2,2, 3,2
// 0,3, 1,3, 2,3, 3,3
fn get_tex_coords(loc: Vec<i32>, total_tiles: i32, square_size: i32) -> [[f32; 2]; 4] {
    let tile_dim: f32 = (total_tiles as f32).sqrt();
    let tile_size: f32 = (square_size as f32).sqrt();
    let per_tile: f32 = (1.0 / tile_dim);

    let top = 1.0 - loc[1] as f32 * per_tile;
    let right = (loc[0] + 1) as f32 * per_tile;
    let bottom = 1.0 - (loc[1] + 1) as f32 * per_tile;
    let left = loc[0] as f32 * per_tile;

    return [
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
        assert!(get_tex_coords(vec![0, 0], 16, 16) == [
          [0.0,  1.0],
          [0.25, 1.0],
          [0.25, 0.75],
          [0.0,  0.75],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_top_right() {
        assert!(get_tex_coords(vec![3, 0], 16, 16) == [
          [0.75, 1.0],
          [1.0,  1.0],
          [1.0,  0.75],
          [0.75, 0.75],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_bottom_right() {
        assert!(get_tex_coords(vec![3, 3], 16, 16) == [
          [0.75, 0.25],
          [1.0,  0.25],
          [1.0,  0.0],
          [0.75, 0.0],
        ]);
    }

    #[test]
    fn it_gets_tex_coords_for_bottom_left() {
        assert!(get_tex_coords(vec![0, 3], 16, 16) == [
          [0.0,  0.25],
          [0.25, 0.25],
          [0.25, 0.0],
          [0.0,  0.0],
        ]);
    }


}
