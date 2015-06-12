use glutin::{
    MouseButton,
    ElementState,
};

use loc::Loc;

#[derive(Debug)]
pub struct Input {
    left_mouse_pressed: bool,
    right_mouse_pressed: bool,
    cursor_loc: Loc,
}

impl Input {
    pub fn new() -> Input {
        return Input {
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            cursor_loc: (0, 0)
        }
    }

    pub fn set_mouse_state(&self, element_state: ElementState, mouse_button: MouseButton) -> Input {
        let left_mouse_pressed = match mouse_button {
            MouseButton::Left => element_state == ElementState::Pressed,
            _ => self.left_mouse_pressed,
        };

        let right_mouse_pressed = match mouse_button {
            MouseButton::Right => element_state == ElementState::Pressed,
            _ => self.right_mouse_pressed,
        };

        return Input {
            left_mouse_pressed: left_mouse_pressed,
            right_mouse_pressed: right_mouse_pressed,
            cursor_loc: self.cursor_loc.clone(),
        }
    }

    pub fn set_mouse_loc(&self, loc: Loc) -> Input {
        return Input {
            left_mouse_pressed: self.left_mouse_pressed.clone(),
            right_mouse_pressed: self.right_mouse_pressed.clone(),
            cursor_loc: loc,
        }
    }
}
