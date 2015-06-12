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

use loc::Loc;
use grid::Grid;
use dood::Dood;
use food::Food;
use wall::Wall;
use pixset::Pixset;
use std::io::Cursor;
use entity::Entity;
use world::gen_world;

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

    let grid = Grid::new(16, 16);

    let mut entities = gen_world();

    // TODO only do walls (not food and player)
    //let blocked = entities.keys().cloned().collect::<Vec<_>>();
    let blocked = vec![];

    loop {
        for (_, entity) in entities.iter_mut() {
            match entity.downcast_mut::<Dood>() {
                Some(dood) => dood.update(&grid, &blocked),
                _ => {}
            }
            match entity.downcast_mut::<Wall>() {
                Some(wall) => wall.update(&grid, &blocked),
                _ => {}
            }
        }

        let (vertices, indices) = square::vertices(&display, &pixset, &entities);

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&vertices, &indices, &program, &uniforms, &draw_parameters).unwrap();

        frame.finish();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }

        thread::sleep_ms(100);
    }
}
