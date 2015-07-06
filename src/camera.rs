use config::SQUARE_SIZE;

use dir::Dir;
use loc::Loc;
use screen_size::ScreenSize;
use window_loc::WindowLoc;
use world_coord::WorldCoord;

#[derive(Debug)]
pub struct Camera {
    screen_size: ScreenSize,
    loc: Loc,
}

impl Camera {
    pub fn new(screen_size: ScreenSize, loc: Loc) -> Camera {
        Camera {
            screen_size: screen_size,
            loc: loc,
        }
    }

    // TODO don't allocate a new one each frame
    pub fn as_mat(&self) -> [[f32; 4]; 4] {
        let x_offset =  (self.loc.x as i32      * SQUARE_SIZE) as f32;
        let y_offset = ((self.loc.y as i32 + 1) * SQUARE_SIZE) as f32;

        let x_o = -(self.screen_size.width  / 2.0) - x_offset;
        let y_o =  (self.screen_size.height / 2.0) - y_offset;

        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_o, y_o, 0.0, 1.0],
        ]
    }

    pub fn pan(&mut self, dir: Dir) {
        println!("{:?}", WorldCoord::from_loc(&self.loc));
        match dir {
            Dir::Right => self.loc.x += 1,
            Dir::Left  => self.loc.x -= 1,
            Dir::Up    => self.loc.y += 1,
            Dir::Down  => self.loc.y -= 1,
            _ => {}
        }
    }

    pub fn to_game_loc(&self, window_loc: WindowLoc) -> Loc {
        let x = window_loc.x as f32 / SQUARE_SIZE as f32;
        let y = window_loc.y as f32 / SQUARE_SIZE as f32;

        Loc {
            x:  (x.trunc() + self.loc.x as f32) as i32,
            y: -(y.trunc() - self.loc.y as f32) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use loc::Loc;
    use window_loc::WindowLoc;
    use screen_size::ScreenSize;
    use super::Camera;

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_four() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -12, y: 10 })
            .to_game_loc(WindowLoc { x: 200.0f64, y: 169.5f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_two() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -7, y: 8 })
            .to_game_loc(WindowLoc { x: 121.0f64, y: 138.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_three() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -7, y: 10 })
            .to_game_loc(WindowLoc { x: 123.0f64, y: 173.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_minus_twelve_twelve_top_left() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -12, y: 12 })
            .to_game_loc(WindowLoc { x: 5.0f64, y: 5.0f64 }) == Loc { x: -12, y: 12 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_top_left() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 })
            .to_game_loc(WindowLoc { x: 1.0f64, y: 1.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_x_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }).as_mat()[3][0] == -128.0);
    }

    #[test]
    fn it_returns_y_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }).as_mat()[3][1] == 112.0);
    }

    #[test]
    fn it_returns_x_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 1, y: 1 }).as_mat()[3][0] == -144.0);
    }

    #[test]
    fn it_returns_y_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 1, y: 1 }).as_mat()[3][1] == 96.0);
    }

    #[test]
    fn it_returns_x_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -1, y: -1 }).as_mat()[3][0] == -112.0);
    }

    #[test]
    fn it_returns_y_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -1, y: -1 }).as_mat()[3][1] == 128.0);
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_sixty_four() {
        let screen_size = ScreenSize { width: 1024.0, height: 1024.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 })
            .to_game_loc(WindowLoc { x: 1015.0f64, y: 1015.0f64 }) == Loc { x: 63, y: -63 });
    }
}
