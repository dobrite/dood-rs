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

mod camera;
mod config;
mod dir;
mod dist;
mod dood;
mod entities;
mod food;
mod grid;
mod input;
mod loc;
mod loc_map;
mod paths;
mod pixset;
mod renderable;
mod shaders;
mod square;
mod state;
mod updatable;
mod utils;
mod window_loc;
mod world;

use std::io::Cursor;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

use camera_controllers::model_view_projection;
use gfx::device::Factory;
use gfx::extra::stream::Stream;
use gfx::render::mesh::ToIndexSlice;
use gfx::extra::factory::FactoryExt;
use gfx::PrimitiveType::TriangleList;
use gfx::PrimitiveType;
use gfx_texture::Texture;

use config::{
    TOTAL_TILES,
    SQUARE_SIZE,
    FRAMES_PER_SECOND,
    UPDATES_PER_SECOND,
};

use camera::Camera;
use pixset::Pixset;
use world::World;
use square::indices;
use input::Input;
use input::Output;

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

use nalgebra::OrthoMat3;

gfx_parameters!(Params {
    mvp@ mvp: [[f32; 4]; 4],
    tex@ tex: gfx::shade::TextureParam<R>,
});

fn main() {
    let width  = 1024.0;
    let height = 1024.0;

    let mut window: PistonWindow = WindowSettings::new(
        "Dood! gets the food!",
        [width as u32, height as u32]
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

    let ortho_projection = *OrthoMat3::new(height, width, 0.0, 100.0).as_mat().as_array();

    let mut uniforms = Params {
        mvp: model_view_projection(mat4_id, mat4_id, ortho_projection),
        tex: (texture, Some(sampler)),
        _r: std::marker::PhantomData,
    };

    //let state = gfx::DrawState::new();
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);
    let pixset = Pixset::new(TOTAL_TILES);
    let clear_data = gfx::ClearData { color: [0.0, 0.0, 0.0, 1.0], depth: 1.0, stencil: 0 };

    let mut world = World::new((width / SQUARE_SIZE as f32) as i32, (height / SQUARE_SIZE as f32) as i32);
    let mut input = Input::new();
    let mut camera = Camera::new(width, height, (-7, 7));

    window.set_max_fps(FRAMES_PER_SECOND);
    window.set_ups(UPDATES_PER_SECOND);

    for e in window {
        e.draw_3d(|stream| {
            let (vertices, indices) = square::vertices(&pixset, &world.renderables);
            stream.clear(clear_data);
            let mesh = &factory.create_mesh(&vertices);
            let tri_list = indices.to_slice(factory, PrimitiveType::TriangleList).clone();
            uniforms.mvp = model_view_projection(mat4_id, camera.as_mat(), ortho_projection);
            stream.draw(&(mesh, tri_list, &program, &uniforms, &state)).unwrap();
        });

        e.update(|_| {
            world.update();
            world.vacuum();
        });

        e.press(|button| {
            match input.press(button) {
                Output::Spawn(window_loc) => world.spawn(camera.to_game_loc(window_loc)),
                Output::CameraMove(dir)   => camera.pan(dir),
                Output::Nothing => {}
            }
        });
        e.release(|button| input.release(button));
        e.mouse_cursor(|x, y| input.mouse_cursor(x, y));
        e.mouse_scroll(|dx, dy| input.mouse_scroll(dx, dy));
        e.mouse_relative(|dx, dy| input.mouse_relative(dx, dy));
        e.text(|text| input.text(text));
    }
}
