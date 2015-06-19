use loc::Loc;
use window_loc::WindowLoc;
use std::mem;

use piston::input::{
    Button,
    Key,
    MouseButton
};

#[derive(Debug)]
pub struct Input{
    mouse_loc: WindowLoc,
    mouse_left: bool,
    mouse_right: bool,
}

impl Input{
    pub fn new() -> Input {
        return Input {
            mouse_loc: (0.0, 0.0),
            mouse_left: false,
            mouse_right: false,
        }
    }

    pub fn press(&mut self, button: Button) -> WindowLoc {
        self.change(button, true);
        return self.mouse_loc
    }

    pub fn release(&mut self, button: Button) {
        self.change(button, false);
    }

    fn change(&mut self, button: Button, state: bool) {
        match button {
            Button::Keyboard(key)             => println!("{:?}", key),
            Button::Mouse(MouseButton::Left)  => self.mouse_left  = state,
            Button::Mouse(MouseButton::Right) => self.mouse_right = state,
            _ => {}
        }
        println!("{:?}", self);
    }

    pub fn mouse_cursor(&mut self, x: f64, y: f64) {
        self.mouse_loc = (x, y);
        println!("{:?}", self);
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
}
