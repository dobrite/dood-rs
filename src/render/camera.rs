
use dir::Dir;
use loc::Loc;
use size::Size;
use screen_size::ScreenSize;
use window_loc::WindowLoc;

#[derive(Debug)]
pub struct Camera {
    screen_size: ScreenSize,
    loc: Loc,
    dim: Size,
}

impl Camera {
    pub fn new(screen_size: ScreenSize, loc: Loc, square_size: i32) -> Camera {
        Camera {
            screen_size: screen_size,
            loc: loc,
            dim: Size {
                width: (screen_size.width  as i32 / square_size),
                height: (screen_size.height as i32 / square_size),
            },
        }
    }

    pub fn get_loc(&self) -> Loc {
        self.loc
    }

    pub fn get_dim(&self) -> Size {
        self.dim
    }

    fn square_size(&self) -> i32 {
        self.screen_size.width as i32 / self.dim.width
    }

    // TODO don't allocate a new one each frame
    pub fn as_mat(&self) -> [[f32; 4]; 4] {
        let x_offset = (self.loc.x      * self.square_size()) as f32;
        let y_offset = ((self.loc.y + 1) * self.square_size()) as f32;

        let x_o = -(self.screen_size.width / 2.0) - x_offset;
        let y_o = (self.screen_size.height / 2.0) - y_offset;

        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_o, y_o, 0.0, 1.0],
        ]
    }

    pub fn pan(&mut self, dir: Dir) -> &mut Self {
        match dir {
            Dir::Right => self.loc.x += 1,
            Dir::Left => self.loc.x -= 1,
            Dir::Up => self.loc.y += 1,
            Dir::Down => self.loc.y -= 1,
        }

        self
    }

    #[allow(dead_code)]
    pub fn to_game_loc(&self, window_loc: WindowLoc) -> Loc {
        let x = window_loc.x as f32 / self.square_size() as f32;
        let y = window_loc.y as f32 / self.square_size() as f32;

        Loc { x: x as i32 + self.loc.x, y: -(y as i32 - self.loc.y) }
    }

    // TODO extract to a trait?
    pub fn to_loc_box(&self) -> (Loc, Loc) {
        (Loc { x: self.loc.x, y: self.loc.y },
         Loc { x: self.loc.x + self.dim.width - 1, y: self.loc.y - self.dim.height + 1 })
    }
}

#[cfg(test)]
mod tests {
    use loc::Loc;
    use dir::Dir;
    use window_loc::WindowLoc;
    use screen_size::ScreenSize;
    use super::Camera;

    #[test]
    fn to_loc_box() {
        let screen_size = ScreenSize { width: 1536.0, height: 1024.0 };
        assert!(Camera::new(screen_size, Loc { x: -50, y: 50 }, 16)
            .to_loc_box() == (Loc { x: -50, y: 50 }, Loc { x: 45, y: -13 }));
    }

    #[test]
    fn to_loc_box_four_four() {
        let screen_size = ScreenSize { width: 64.0, height: 64.0 };
        assert!(Camera::new(screen_size, Loc { x: 10, y: 10 }, 16)
            .to_loc_box() == (Loc { x: 10, y: 10 }, Loc { x: 13, y: 7 }));
    }

    #[test]
    fn pan_up() {
        let screen_size = ScreenSize { width: 64.0, height: 64.0 };
        let mut camera = Camera::new(screen_size, Loc { x: 0, y: 0 }, 16);
        assert!(camera.pan(Dir::Up).get_loc() == Loc { x: 0, y: 1 });
    }

    #[test]
    fn pan_down() {
        let screen_size = ScreenSize { width: 64.0, height: 64.0 };
        let mut camera = Camera::new(screen_size, Loc { x: 0, y: 0 }, 16);
        assert!(camera.pan(Dir::Down).get_loc() == Loc { x: 0, y: -1 });
    }

    #[test]
    fn pan_right() {
        let screen_size = ScreenSize { width: 64.0, height: 64.0 };
        let mut camera = Camera::new(screen_size, Loc { x: 0, y: 0 }, 16);
        assert!(camera.pan(Dir::Right).get_loc() == Loc { x: 1, y: 0 });
    }

    #[test]
    fn pan_left() {
        let screen_size = ScreenSize { width: 64.0, height: 64.0 };
        let mut camera = Camera::new(screen_size, Loc { x: 0, y: 0 }, 16);
        assert!(camera.pan(Dir::Left).get_loc() == Loc { x: -1, y: 0 });
    }


    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_four() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -12, y: 10 }, 16)
            .to_game_loc(WindowLoc { x: 200.0f64, y: 169.5f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_two() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -7, y: 8 }, 16)
            .to_game_loc(WindowLoc { x: 121.0f64, y: 138.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_three() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -7, y: 10 }, 16)
            .to_game_loc(WindowLoc { x: 123.0f64, y: 173.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_minus_twelve_twelve_top_left() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -12, y: 12 }, 16)
            .to_game_loc(WindowLoc { x: 5.0f64, y: 5.0f64 }) == Loc { x: -12, y: 12 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_top_left() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }, 16)
            .to_game_loc(WindowLoc { x: 1.0f64, y: 1.0f64 }) == Loc { x: 0, y: 0 });
    }

    #[test]
    fn it_returns_game_coords_for_window_loc_zero_zero_bottom_right_sixty_four() {
        let screen_size = ScreenSize { width: 1024.0, height: 1024.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }, 16)
            .to_game_loc(WindowLoc { x: 1015.0f64, y: 1015.0f64 }) == Loc { x: 63, y: -63 });
    }

    #[test]
    fn it_returns_x_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }, 16).as_mat()[3][0] == -128.0);
    }

    #[test]
    fn it_returns_y_for_loc_zero_zero_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 0, y: 0 }, 16).as_mat()[3][1] == 112.0);
    }

    #[test]
    fn it_returns_x_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 1, y: 1 }, 16).as_mat()[3][0] == -144.0);
    }

    #[test]
    fn it_returns_y_for_loc_one_one_square_size_sixteen_and_two_fifty_six() {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: 1, y: 1 }, 16).as_mat()[3][1] == 96.0);
    }

    #[test]
    fn it_returns_x_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six
        () {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -1, y: -1 }, 16).as_mat()[3][0] == -112.0);
    }

    #[test]
    fn it_returns_y_for_loc_minus_one_minus_one_square_size_sixteen_and_two_fifty_six
        () {
        let screen_size = ScreenSize { width: 256.0, height: 256.0 };
        assert!(Camera::new(screen_size, Loc { x: -1, y: -1 }, 16).as_mat()[3][1] == 128.0);
    }

    #[test]
    fn square_size_it_returns_for_sixteen() {
        let screen_size = ScreenSize { width: 1024.0, height: 1024.0 };
        assert!(Camera::new(screen_size, Loc { x: -1, y: -1 }, 16).square_size() == 16);
    }
}
