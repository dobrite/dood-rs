use loc::Loc;
use window_loc::WindowLoc;
use dir::Dir;

use piston::input::{
    Button,
    Key,
    MouseButton
};

pub enum Output {
    CameraMove(Dir),
    Spawn(WindowLoc),
    Nothing,
}

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

    pub fn press(&mut self, button: Button) -> Output {
        return self.change(button, true)
    }

    pub fn release(&mut self, button: Button) {
        self.change(button, false);
    }

    fn change(&mut self, button: Button, state: bool) -> Output {
        let out = match button {
            Button::Keyboard(key)             => {
                match key {
                    Key::Up    => Output::CameraMove(Dir::Up),
                    Key::Down  => Output::CameraMove(Dir::Down),
                    Key::Right => Output::CameraMove(Dir::Right),
                    Key::Left  => Output::CameraMove(Dir::Left),
                    _ => Output::Nothing
                }
            },
            Button::Mouse(MouseButton::Left)  => {
                self.mouse_left  = state;
                Output::Spawn(self.mouse_loc)
            },
            Button::Mouse(MouseButton::Right) => {
                self.mouse_right = state;
                Output::Nothing
            },
            _ => Output::Nothing
        };
        return out
    }

    pub fn mouse_cursor(&mut self, x: f64, y: f64) {
        self.mouse_loc = (x, y);
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
