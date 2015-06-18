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

mod config;
mod dood;
mod entities;
mod entity;
mod food;
mod grid;
mod input;
mod loc;
mod paths;
mod pixset;
mod renderable;
mod shaders;
mod square;
mod state;
mod updatable;
mod wall;
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
use input::Input;

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

    let input = Input::new();

    for e in window {
        e.draw_3d(|stream| {
            let (vertices, indices) = square::vertices(&pixset, &world.entities);
            stream.clear(clear_data);
            let mesh = &factory.create_mesh(&vertices);
            let tri_list = indices.to_slice(factory, PrimitiveType::TriangleList).clone();
            stream.draw(&(mesh, tri_list, &program, &uniforms, &state)).unwrap();
        });

        e.update(|_| println!("update!"));
        e.press(|button| input.press(button));
        e.release(|button| input.release(button));
        e.mouse_cursor(|x, y| input.mouse_cursor(x, y));
        e.mouse_scroll(|dx, dy| input.mouse_scroll(dx, dy));
        e.mouse_relative(|dx, dy| input.mouse_relative(dx, dy));
        e.text(|text| input.text(text));
    }
}
