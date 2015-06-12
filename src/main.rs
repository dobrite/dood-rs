#[macro_use]
extern crate glium;
extern crate piston;
extern crate glutin;
extern crate glutin_window;
extern crate rand;
extern crate image;

mod shaders;
mod square;
mod grid;
mod dood;
mod food;
mod wall;
mod paths;
mod renderable;
mod pixset;
mod config;
mod updatable;
mod entity;
mod entities;
mod loc;
mod state;
mod world;
mod input;

use std::collections::HashMap;
use std::any::Any;
use std::thread;

use piston::window::WindowSettings;
use piston::event::*; // TODO not this
use glutin_window::GlutinWindow as Window;

use glium::{
    DisplayBuild,
    Surface,
    Program,
};

use glium::draw_parameters::{
    LinearBlendingFactor,
    BlendingFunction,
};

use config::{
    SQUARE_SIZE,
    TOTAL_TILES,
    //UPDATES_PER_SECOND,
};

use input::Input;
use loc::Loc;
use grid::Grid;
use dood::Dood;
use food::Food;
use wall::Wall;
use pixset::Pixset;
use std::io::Cursor;
use entity::Entity;
use world::World;

fn main() {
    let width = 256.0;
    let height = 256.0;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_title(format!("Dood! gets the food!"))
        .with_vsync()
        .build_glium()
        .unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/tileset.png")[..]), image::PNG).unwrap();
    let tileset = glium::texture::Texture2d::new(&display, image);
    let pixset = Pixset::new(TOTAL_TILES);
    let program = Program::from_source(&display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap();

    let draw_parameters = glium::DrawParameters {
        blending_function: Some(BlendingFunction::Addition {
            source: LinearBlendingFactor::SourceAlpha,
            destination: LinearBlendingFactor::OneMinusSourceAlpha,
        }),
        .. Default::default()
    };

    let uniforms = uniform! {
        view_transform: [
            [ 1.0 / width, 0.0         , 0.0, 0.0],
            [ 0.0        , 1.0 / height, 0.0, 0.0],
            [ 0.0        , 0.0         , 1.0, 0.0],
            [-1.0        , 1.0         , 0.0, 1.0f32]
        ],
        tileset: &tileset,
    };

    let mut world = World::new();
    let mut input = Input::new();

    loop {
        for event in display.poll_events() {
            // TODO event_manager(event); to get this out of here
            match event {
                glutin::Event::Closed => return,
                glutin::Event::MouseInput(element_state, mouse_button) => input = input.set_mouse_state(element_state, mouse_button),
                glutin::Event::MouseMoved(loc) => input = input.set_mouse_loc(loc),
                _ => ()
                //event => state.handle(&event) // this was on a paste
            }
        }

        world.update();

        let (vertices, indices) = square::vertices(&display, &pixset, &world.entities);
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&vertices, &indices, &program, &uniforms, &draw_parameters).unwrap();
        frame.finish();

        println!("{:?}", input);

        thread::sleep_ms(100); // TODO lol
    }
}
