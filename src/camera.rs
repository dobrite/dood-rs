use loc::Loc;
use config::SQUARE_SIZE;

#[derive(Debug)]
pub struct Camera {
    height: f32,
    width: f32,
    loc: (i32, i32),
}

impl Camera {
    pub fn new(height: f32, width: f32, loc: (i32, i32)) -> Camera {
        return Camera {
            height: height,
            width: width,
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
}

#[cfg(test)]
mod tests {
    use super::Camera;

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
}
