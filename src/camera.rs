use config::SQUARE_SIZE;

use dir::Dir;
use loc::Loc;
use window_loc::WindowLoc;

#[derive(Debug)]
pub struct Camera {
    width: f32,
    height: f32,
    loc: (i32, i32),
}

impl Camera {
    pub fn new(width: f32, height: f32, loc: (i32, i32)) -> Camera {
        return Camera {
            width: width,
            height: height,
            loc: loc,
        }
    }

    pub fn as_mat(&self) -> [[f32; 4]; 4] {
        let x_offset =  (self.loc.0 as i32      * SQUARE_SIZE) as f32;
        let y_offset = ((self.loc.1 as i32 + 1) * SQUARE_SIZE) as f32;

        let x_o = -(self.width  / 2.0) - x_offset;
        let y_o =  (self.height / 2.0) - y_offset;

        return [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_o, y_o, 0.0, 1.0],
        ]
    }

    pub fn pan(&mut self, dir: Dir) {
        match dir {
            Dir::Up    => self.loc.1 = self.loc.1 + 1,
            Dir::Down  => self.loc.1 = self.loc.1 - 1,
            Dir::Right => self.loc.0 = self.loc.0 + 1,
            Dir::Left  => self.loc.0 = self.loc.0 - 1,
            _ => {}
        }
    }

    pub fn to_game_loc(&self, window_loc: WindowLoc) -> Loc {
        let x = (window_loc.x as f32 / SQUARE_SIZE as f32);
        let y = (window_loc.y as f32 / SQUARE_SIZE as f32);

        return (
             ((x.trunc() + self.loc.0 as f32).round() as i32),
            (-(y.trunc() - self.loc.1 as f32).round() as i32)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_four() {
        assert!(Camera::new(256.0, 256.0, (-12, 10)).to_game_loc((200.0f64, 169.5f64)) == (0, 0));
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_two() {
        assert!(Camera::new(256.0, 256.0, (-7, 8)).to_game_loc((121.0f64, 138.0f64)) == (0, 0));
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_three() {
        assert!(Camera::new(256.0, 256.0, (-7, 10)).to_game_loc((123.0f64, 173.0f64)) == (0, 0));
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_minus_twelve_twelve_top_left() {
        assert!(Camera::new(256.0, 256.0, (-12, 12)).to_game_loc((5.0f64, 5.0f64)) == (-12, 12));
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_top_left() {
        assert!(Camera::new(256.0, 256.0, (0, 0)).to_game_loc((1.0f64, 1.0f64)) == (0, 0));
    }

    #[test]
    fn it_returns_x_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (0, 0)).as_mat()[3][0] == -128.0);
    }

    #[test]
    fn it_returns_y_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (0, 0)).as_mat()[3][1] == 112.0);
    }

    #[test]
    fn it_returns_x_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (1, 1)).as_mat()[3][0] == -144.0);
    }

    #[test]
    fn it_returns_y_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (1, 1)).as_mat()[3][1] == 96.0);
    }

    #[test]
    fn it_returns_x_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (-1, -1)).as_mat()[3][0] == -112.0);
    }

    #[test]
    fn it_returns_y_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six() {
        assert!(Camera::new(256.0, 256.0, (-1, -1)).as_mat()[3][1] == 128.0);
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_sixty_four() {
        assert!(Camera::new(1024.0, 1024.0, (0, 0)).to_game_loc((1015.0f64, 1015.0f64)) == (63, -63));
    }
}
