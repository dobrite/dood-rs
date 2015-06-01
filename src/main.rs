#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;
extern crate image;

mod shaders;
mod square;
mod grid;
mod dood;
mod food;
mod wall;
mod paths;
mod has_loc;
mod renderable;
mod pixset;
mod config;

use std::thread;
use std::sync::{
    Arc,
    Mutex
};

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
    UPDATES_PER_SECOND,
};

use grid::Grid;
use dood::Dood;
use pixset::Pixset;
use std::io::Cursor;

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
    let pixset = Pixset::new(TOTAL_TILES, SQUARE_SIZE);
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

    // Arc is needed until thread::scoped is stable
    let grid = Arc::new(Mutex::new(Grid::new(16, 16, SQUARE_SIZE as f32)));

    {
        let grid = grid.clone();
        // Spawn off thread to update the grid. Main thread will be in charge of rendering
        thread::spawn(move || {
            loop {
                thread::sleep_ms(1000 / UPDATES_PER_SECOND as u32);
                grid.lock().unwrap().update();
            }
        });
    }

    loop {
        let (vertices, indices) = {
            let grid = grid.lock().unwrap();
            square::vertices(&display, &pixset, &grid.stuffs)
        };

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
    }
}
