#[macro_use]
extern crate gfx;

extern crate piston;
extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate image;
extern crate fps_counter;
extern crate hprof;
extern crate camera_controllers;
extern crate nalgebra;

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

use std::io::Cursor;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

use piston::input::{
    Button,
    Key
};

use camera_controllers::model_view_projection;
use gfx::device::Factory;
use gfx::extra::stream::Stream;
use gfx::render::mesh::ToIndexSlice;
use gfx::extra::factory::FactoryExt;
use gfx::PrimitiveType::TriangleList;
use gfx::PrimitiveType;
use gfx_texture::Texture;

use config::TOTAL_TILES;

use pixset::Pixset;
use world::World;
use square::indices;

use piston_window::{
    EventLoop,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    PressEvent,
    ReleaseEvent,
    TextEvent,
    UpdateEvent,
};

use nalgebra::{
    Mat4,
    OrthoMat3,
};

gfx_parameters!(Params {
    view_transform@ view_transform: [[f32; 4]; 4],
    tex@ tex: gfx::shade::TextureParam<R>,
});

fn main() {
    let width = 256.0;
    let height = 256.0;

    let mut window: PistonWindow = WindowSettings::new(
        "Dood! gets the food!",
        [height as u32, width as u32]
    ).exit_on_esc(true).into();

    let ref mut factory = window.factory.borrow().clone();

    let program = {
        let vertex = gfx::ShaderSource {
            glsl_140: Some(shaders::VERTEX),
            .. gfx::ShaderSource::empty()
        };
        let fragment = gfx::ShaderSource {
            glsl_140: Some(shaders::FRAGMENT),
            .. gfx::ShaderSource::empty()
        };
        factory.link_program_source(vertex, fragment).unwrap()
    };

    let image = image::load(Cursor::new(&include_bytes!("../assets/tileset.png")[..]), image::PNG).unwrap();
    let texture = Texture::from_image(factory, &image.to_rgba(), true, false, false).handle();
    let sampler_info = gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Bilinear, gfx::tex::WrapMode::Clamp);
    let sampler = factory.create_sampler(sampler_info);

    let mat4_id = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let ortho_projection = *OrthoMat3::new(height, width, 0.0, 1.0).as_mat().as_array();

    let uniforms = Params {
        view_transform: model_view_projection(mat4_id, mat4_id, ortho_projection),
        tex: (texture, Some(sampler)),
        _r: std::marker::PhantomData,
    };

    //let state = gfx::DrawState::new();
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);
    let world = World::new();
    let pixset = Pixset::new(TOTAL_TILES);
    let clear_data = gfx::ClearData { color: [0.0, 0.0, 0.0, 1.0], depth: 1.0, stencil: 0 };

    window.set_max_fps(30);
    window.set_ups(1);

    for e in window {
        e.draw_3d(|stream| {
            let (vertices, indices) = square::vertices(&pixset, &world.entities);
            stream.clear(clear_data);
            stream.draw(&(&factory.create_mesh(&vertices), indices.to_slice(factory, PrimitiveType::TriangleList).clone(), &program, &uniforms, &state)).unwrap();
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
        };
        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{:?}'", key),
                Button::Mouse(button) => println!("Released mouse button '{:?}'", button),
            }
        };
        e.mouse_cursor(|x, y| {
            let cursor = [x, y];
            println!("Mouse moved '{} {}'", x, y);
        });
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.update(|_|
          { println!("update!");
        });
    }
}
