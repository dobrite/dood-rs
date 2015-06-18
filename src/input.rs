use loc::Loc;
use std::mem;

use piston::input::{
    Button,
    Key
};

#[derive(Debug)]
pub struct Input {
    current: InputState,
    previous: InputState,
}

#[derive(Debug)]
struct InputState {
    right_mouse: bool,
    left_mouse: bool,
    cursor_loc: Loc,
}

impl InputState {
    pub fn new() -> InputState {
        return InputState {
            right_mouse: false,
            left_mouse: false,
            cursor_loc: (0, 0),
        }
    }
}

impl Input {
    pub fn new() -> Input {
        return Input {
            current: InputState::new(),
            previous: InputState::new(),
        }
    }

    pub fn swap(&mut self) {
        mem::swap(&mut self.previous, &mut self.current);
    }

    pub fn press(&self, button: Button) {
        println!("Pressed button {:?}", button);
    }

    pub fn release(&self, button: Button) {
        println!("Released button {:?}", button);
    }

    pub fn mouse_cursor(&self, x: f64, y: f64) {
        println!("Mouse moved '{} {}'", x, y);
    }

    pub fn mouse_scroll(&self, dx: f64, dy: f64) {
        println!("Scrolled mouse '{}, {}'", dx, dy);
    }

    pub fn mouse_relative(&self, dx: f64, dy: f64) {
        println!("Relative mouse moved '{} {}'", dx, dy);
    }

    pub fn text(&self, text: &str) {
        println!("Typed '{}'", text);
    }

    //pub fn set_mouse_state(&self, element_state: ElementState, mouse_button: MouseButton) -> Input {
    //    let left_mouse_pressed = match mouse_button {
    //        MouseButton::Left => element_state == ElementState::Pressed,
    //        _ => self.left_mouse_pressed,
    //    };

    //    let right_mouse_pressed = match mouse_button {
    //        MouseButton::Right => element_state == ElementState::Pressed,
    //        _ => self.right_mouse_pressed,
    //    };

    //    return Input {
    //        left_mouse_pressed: left_mouse_pressed,
    //        right_mouse_pressed: right_mouse_pressed,
    //        cursor_loc: self.cursor_loc.clone(),
    //    }
    //}

    //pub fn set_mouse_loc(&self, loc: Loc) -> Input {
    //    return Input {
    //        left_mouse_pressed: self.left_mouse_pressed.clone(),
    //        right_mouse_pressed: self.right_mouse_pressed.clone(),
    //        cursor_loc: loc,
    //    }
    //}
}
