// A port of restrivtive precise angle showcasting by Dominik Marczuk
// http://www.roguebasin.com/index.php?title=Restrictive_Precise_Angle_Shadowcasting

// A port of fov.ts in yendor.ts by jice-nospam
// https://github.com/jice-nospam/yendor.ts/blob/master/src/yendor/fov.ts

//pub struct Grid<'a> {
//  grid: &'a[&'a[i32]]
//}
//
//impl <'a>Grid<'a> {
//  fn new(g: &'a[&'a[i32]]) -> Grid<'a> {
//    Grid {
//      grid: g
//    }
//  }
//
//  fn print(&self) {
//    println!("{:?}", self.grid);
//  }
//}
// //let ys: [i32; 500] = [0; 500];
//fn main() {
//  let t = &[&[5; 2][..], &[5; 2][..]];
//  let g = Grid::new(t);
//  g.print();
//}

use std::cell::RefCell;
use std::cmp;
use std::fmt;
use std::rc::Weak;

use super::vertex::Vertex;
use super::flags::{NONE, IN_FOV, TRANSPARENT, Flags};

use size::Size;
use pixset::Pixset;

use chunks::Chunks;

// TODO maybe one day
// http://stackoverflow.com/a/29531983 (accept mix of vecs and slices)
pub struct Fov {
    start_angle: Vec<f64>,
    end_angle: Vec<f64>,
    pub width: i32, // TODO dont leave pub and switch to Size
    height: i32,
}

//impl fmt::Debug for Fov {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let rc_option = self.entity.upgrade();
//        let ref_cell   = rc_option.unwrap();
//        let ent       = ref_cell.borrow();
//        write!(f, "{:?}", ent.get_loc())
//    }
//}


impl Fov {
    pub fn new(width: i32, height: i32) -> Fov {
        let mut start_angle = Vec::new();
        let mut end_angle = Vec::new();
        start_angle.resize(1000, 0.0); // TODO fix
        end_angle.resize(1000, 1.0); // TODO fix

        // TODO use 1d for FOV
        Fov {
            start_angle: start_angle,
            end_angle: end_angle,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn compute_fov(&mut self, x: i32, y: i32, max_radius: i32, light_walls: bool, mut flags: &mut Vec<Flags>) {
        flags[(self.width * y + x) as usize].insert(IN_FOV);
        self.compute_quadrant_vertical(x, y, max_radius, light_walls, 1, 1, flags);
        self.compute_quadrant_horizontal(x, y, max_radius, light_walls, 1, 1, flags);
        self.compute_quadrant_vertical(x, y, max_radius, light_walls, 1, -1, flags);
        self.compute_quadrant_horizontal(x, y, max_radius, light_walls, 1, -1, flags);
        self.compute_quadrant_vertical(x, y, max_radius, light_walls, -1, 1, flags);
        self.compute_quadrant_horizontal(x, y, max_radius, light_walls, -1, 1, flags);
        self.compute_quadrant_vertical(x, y, max_radius, light_walls, -1, -1, flags);
        self.compute_quadrant_horizontal(x, y, max_radius, light_walls, -1, -1, flags);
    }

    fn is_transparent(&self, x: i32, y: i32, flags: &Vec<Flags>) -> bool {
        flags[(self.width * y + x) as usize].contains(TRANSPARENT)
    }

    fn set_transparent(&mut self, x: i32, y: i32, value: bool, mut flags: &mut Vec<Flags>) {
        if value {
            flags[(self.width * y + x) as usize].insert(TRANSPARENT)
        } else {
            flags[(self.width * y + x) as usize].remove(TRANSPARENT)
        }
    }

    fn is_in_fov(&self, x: i32, y: i32, flags: &Vec<Flags>) -> bool {
        flags[(self.width * y + x) as usize].contains(IN_FOV)
    }

    fn can_see(&self, x: i32, y: i32, flags: &Vec<Flags>) -> bool {
        self.is_in_fov(x, y, flags) && self.is_transparent(x, y, flags)
    }

    fn compute_quadrant_vertical(&mut self,
                                 x_pov: i32,
                                 y_pov: i32,
                                 max_radius: i32,
                                 light_walls: bool,
                                 dx: i32,
                                 dy: i32,
                                 mut flags: &mut Vec<Flags>) {
        let mut y = y_pov + dy;
        let mut done = false;
        let mut iteration = 1;
        let mut min_angle = 0.0;
        let mut last_line_obstacle_count = 0;
        let mut total_obstacle_count = 0;
        while !done && y >= 0 && y < self.height {
            let slope_per_cell = (iteration as f64).recip();
            let half_slope = slope_per_cell * 0.5;
            let x_min = cmp::max(0, x_pov - iteration);
            let x_max = cmp::min(self.width - 1, x_pov + iteration);
            let mut processed_cell = ((min_angle + half_slope) / slope_per_cell).floor() as i32;
            let mut x = x_pov + processed_cell * dx;
            done = true;
            while x >= x_min && x <= x_max {
                let center_slope = processed_cell as f64 * slope_per_cell;
                let start_slope = center_slope - half_slope;
                let end_slope = center_slope + half_slope;
                let mut visible = true;
                let mut extended = false;
                if last_line_obstacle_count > 0 && !self.is_in_fov(x, y, flags) {
                    let mut idx = 0;
                    if visible && !self.can_see(x, y - dy, flags) && x - dx >= 0 && x - dx < self.width &&
                       !self.can_see(x - dx, y - dy, flags) {
                        visible = false;
                    } else {
                        while visible && idx < last_line_obstacle_count {
                            if self.start_angle[idx] > end_slope ||
                               self.end_angle[idx] < start_slope {
                                idx += 1;
                            } else {
                                if self.is_transparent(x, y, flags) {
                                    if center_slope > self.start_angle[idx] &&
                                       center_slope < self.end_angle[idx] {
                                        visible = false;
                                    }
                                } else {
                                    if start_slope >= self.start_angle[idx] &&
                                       end_slope <= self.end_angle[idx] {
                                        visible = false;
                                    } else {
                                        self.start_angle[idx] = self.start_angle[idx].min(start_slope);
                                        self.end_angle[idx] = self.end_angle[idx].max(end_slope);
                                        extended = true;
                                    }
                                }
                                idx += 1;
                            }
                        }
                    }
                }
                if visible {
                    flags[(self.width * y + x) as usize].insert(IN_FOV);
                    done = false;
                    // if the cell is opaque, block the adjacent slopes
                    if !self.is_transparent(x, y, flags) {
                        if min_angle >= start_slope {
                            min_angle = end_slope;
                            // if min_angle is applied to the last cell in line, nothing more
                            // needs to be checked.
                            if processed_cell == iteration {
                                done = true;
                            }
                        } else if !extended {
                            self.start_angle[total_obstacle_count] = start_slope;
                            self.end_angle[total_obstacle_count] = end_slope;
                            total_obstacle_count += 1;
                        }
                        if !light_walls {
                            flags[(self.width * y + x) as usize].remove(IN_FOV);
                        }
                    }
                }
                processed_cell += 1;
                x += dx;
            }
            if iteration == max_radius {
                done = true;
            }
            iteration += 1;
            last_line_obstacle_count = total_obstacle_count;
            y += dy;
            if y < 0 || y >= self.height {
                done = true;
            }
        }
    }

    fn compute_quadrant_horizontal(&mut self,
                                   x_pov: i32,
                                   y_pov: i32,
                                   max_radius: i32,
                                   light_walls: bool,
                                   dx: i32,
                                   dy: i32,
                                   mut flags: &mut Vec<Flags>) {
        let mut x = x_pov + dx;
        let mut done = false;
        let mut iteration = 1;
        let mut min_angle = 0.0;
        let mut last_line_obstacle_count = 0;
        let mut total_obstacle_count = 0;
        while !done && x >= 0 && x < self.width {
            let slope_per_cell = (iteration as f64).recip();
            let half_slope = slope_per_cell * 0.5;
            let y_min = cmp::max(0, y_pov - iteration);
            let y_max = cmp::min(self.height - 1, y_pov + iteration);
            let mut processed_cell = ((min_angle + half_slope) / slope_per_cell).floor() as i32;
            let mut y = y_pov + processed_cell * dy;
            done = true;
            while y >= y_min && y <= y_max {
                let center_slope = processed_cell as f64 * slope_per_cell;
                let start_slope = center_slope - half_slope;
                let end_slope = center_slope + half_slope;
                let mut visible = true;
                let mut extended = false;
                if last_line_obstacle_count > 0 && !self.is_in_fov(x, y, flags) {
                    let mut idx = 0;
                    if visible && !self.can_see(x - dx, y, flags) && y - dy >= 0 && y - dy < self.height &&
                       !self.can_see(x - dx, y - dy, flags) {
                        visible = false;
                    } else {
                        while visible && idx < last_line_obstacle_count {
                            if self.start_angle[idx] > end_slope ||
                               self.end_angle[idx] < start_slope {
                                idx += 1;
                            } else {
                                if self.is_transparent(x, y, flags) {
                                    if center_slope > self.start_angle[idx] &&
                                       center_slope < self.end_angle[idx] {
                                        visible = false;
                                    }
                                } else {
                                    if start_slope >= self.start_angle[idx] &&
                                       end_slope <= self.end_angle[idx] {
                                        visible = false;
                                    } else {
                                        self.start_angle[idx] = self.start_angle[idx].min(start_slope);
                                        self.end_angle[idx] = self.end_angle[idx].max(end_slope);
                                        extended = true;
                                    }
                                }
                                idx += 1;
                            }
                        }
                    }
                }
                if visible {
                    flags[(self.width * y + x) as usize].insert(IN_FOV);
                    done = false;
                    // if the cell is opaque, block the adjacent slopes
                    if !self.is_transparent(x, y, flags) {
                        if min_angle >= start_slope {
                            min_angle = end_slope;
                            // if min_angle is applied to the last cell in line, nothing more
                            // needs to be checked.
                            if processed_cell == iteration {
                                done = true;
                            }
                        } else if !extended {
                            self.start_angle[total_obstacle_count] = start_slope;
                            self.end_angle[total_obstacle_count] = end_slope;
                            total_obstacle_count += 1;
                        }
                        if !light_walls {
                            flags[(self.width * y + x) as usize].remove(IN_FOV);
                        }
                    }
                }
                processed_cell += 1;
                y += dy;
            }
            if iteration == max_radius {
                done = true;
            }
            iteration += 1;
            last_line_obstacle_count = total_obstacle_count;
            x += dx;
            if x < 0 || x >= self.width {
                done = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fov;
    use super::super::flags::IN_FOV;

    use std::rc::Rc;
    use std::cell::RefCell;

    use loc::Loc;

    #[test]
    fn new_it_returns_fov() {
        let fov = Fov::new(2, 2);
        assert!(true);
    }

    #[test]
    fn clear_fov_it_clears_fov() {
        let mut fov = Fov::new(2, 2);
        fov.clear_fov();
        assert!(fov.flags.iter().all(|flag| *flag != IN_FOV));
    }

    #[test]
    fn get_width_it_gets_the_width() {
        let mut fov = Fov::new(2, 2);
        assert!(fov.get_width() == 2);
    }

    #[test]
    fn get_height_it_gets_the_height() {
        let mut fov = Fov::new(2, 2);
        assert!(fov.get_height() == 2);
    }

    #[test]
    fn is_transparent_it_gets_transparency_for_indices() {
        let mut fov = Fov::new(2, 2);
        assert!(fov.is_transparent(1, 1) == true);
    }

    #[test]
    fn set_transparent_it_sets_transparency_for_indices() {
        let mut fov = Fov::new(2, 2);
        fov.set_transparent(1, 1, false);
        assert!(fov.is_transparent(1, 1) == false);
    }

    #[test]
    fn is_in_fov_it_returns_value_at_indices() {
        let mut fov = Fov::new(2, 2);
        assert!(fov.is_in_fov(1, 1) == false);
    }

    #[test]
    fn can_see_it_returns_and_of_is_in_fov_and_is_transparent() {
        let mut fov = Fov::new(2, 2);
        assert!(fov.can_see(1, 1) == false);
    }
}
