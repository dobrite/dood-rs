#![feature(vec_resize, rc_weak, clone_from_slice)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate gfx;

extern crate ai_behavior;
extern crate camera_controllers;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate image;
extern crate piston;
extern crate piston_window;
extern crate rand;

mod action;
mod brain;
mod camera;
mod chunk;
mod chunk_loc;
mod config;
mod dir;
mod dist;
mod cascadecs;
mod food;
mod fov;
mod grid;
mod indices;
mod input;
mod loc;
mod path;
mod pixset;
mod scratch;
mod screen_size;
mod shaders;
mod size;
mod state;
mod terrain;
mod utils;
mod window_loc;
mod chunks;
mod vertex;
mod world_coord;

use std::io::Cursor;

use piston_window::{PistonWindow, WindowSettings};

use camera_controllers::model_view_projection;
use gfx::device::Factory;
use gfx::extra::stream::Stream;
use gfx::render::mesh::ToIndexSlice;
use gfx::extra::factory::FactoryExt;
use gfx::PrimitiveType::TriangleList;
use gfx::PrimitiveType;
use gfx_texture::{Texture, TextureSettings};

use loc::Loc;
use size::Size;
use brain::Brain;
use camera::Camera;
use food::Food;
use pixset::Pixset;
use chunks::Chunks;
use input::Input;
use input::Output;
use pixset::Pix;
use scratch::Scratch;
use screen_size::ScreenSize;
use world_coord::WorldCoord;

use cascadecs::entity::Entity;
use cascadecs::components::Components;
use cascadecs::processes::Processes;

use piston_window::{EventLoop, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, PressEvent,
                    ReleaseEvent, TextEvent, UpdateEvent};

gfx_parameters!(Params {
    mvp@ mvp: [[f32; 4]; 4],
    tex@ tex: gfx::shade::TextureParam<R>,
});

fn main() {
    let screen_size = ScreenSize { width: 1536.0 /* 96 */, height: 1024.0 /* 64 */ };

    let mut window: PistonWindow = WindowSettings::new(
        "Dood! gets the food!",
        [screen_size.width as u32, screen_size.height as u32]
    ).exit_on_esc(true).build().unwrap();

    let ref mut factory = window.factory.borrow().clone();

    let program = {
        let vertex = gfx::ShaderSource {
            glsl_140: Some(shaders::VERTEX),
            // .
            ..gfx::ShaderSource::empty()
        };
        let fragment = gfx::ShaderSource {
            glsl_140: Some(shaders::FRAGMENT),
            // .
            ..gfx::ShaderSource::empty()
        };
        factory.link_program_source(vertex, fragment).unwrap()
    };

    let cursor = Cursor::new(&include_bytes!("../assets/tileset.png")[..]);
    let image = image::load(cursor, image::PNG).unwrap();
    let texture_settings = TextureSettings::new().convert_gamma(true);
    let texture = Texture::from_image(
        factory, &image.to_rgba(), &texture_settings).unwrap().handle();
    let sampler_info = gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Scale,
                                                  gfx::tex::WrapMode::Clamp);
    let sampler = factory.create_sampler(sampler_info);

    let mat4_id = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let z_far = 100.0;
    let o_w = 2.0 / screen_size.width;
    let o_h = 2.0 / screen_size.height;
    let o_z = 2.0 / -(z_far);

    let ortho_projection = [
        [o_w, 0.0,  0.0, 0.0],
        [0.0, o_h,  0.0, 0.0],
        [0.0, 0.0,  o_z, 0.0],
        [0.0, 0.0, -1.0, 1.0],
    ];

    let mut uniforms = Params {
        mvp: model_view_projection(mat4_id, mat4_id, ortho_projection),
        tex: (texture, Some(sampler)),
        _r: std::marker::PhantomData,
    };

    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);
    let pixset = Pixset::new(config::TOTAL_TILES);
    let clear_data = gfx::ClearData { color: [0.0, 0.0, 0.0, 1.0], depth: 1.0, stencil: 0 };

    let mut chunks = Chunks::new(Size {
            width: config::CHUNK_WIDTH,
            height: config::CHUNK_HEIGHT,
        });
    let mut components = Components::new();
    let mut input = Input::new();
    let mut camera = Camera::new(screen_size, Loc { x: -32, y: 16 }, config::SQUARE_SIZE);
    let processes = Processes::new();
    let camera_dim = camera.get_dim();
    let mut scratch = {
        let size = Size { width: camera_dim.width * 2, height: camera_dim.height * 3 };
        let loc = Loc { x: -80, y: 80 };
        Scratch::new(loc, size).inflate(&mut chunks)
    };

    {
        let loc = Loc { x: 20, y: 0 };
        let ref mut chunk = chunks.get_chunk(&WorldCoord::from_loc(&loc).get_chunk_loc());
        // TODO some sort of "blueprint"
        let entity = Entity::new();
        let color = [1.0, 1.0, 1.0];
        components.new_brain_component(entity, Brain::Dood);
        components.new_render_component(entity, Pix::Dood, color);
        components.new_position_component(entity, loc);
        components.new_hunger_component(entity, 100 as u16, 1 as u8);
        components.new_fov_component(entity, 10);
        chunk.insert_entity(entity); // maybe chunks.add_entity_to_chunk?
        // TODO do bounds checking
        scratch.insert_into_entities(entity);
    }

    window.set_max_fps(config::FRAMES_PER_SECOND);
    window.set_ups(config::UPDATES_PER_SECOND);

    // maybe a less magic event loop?
    // http://jadpole.github.io/arcaders/arcaders-1-3/
    for e in window {
        e.update(|dt| {
            let delta = processes.update(&components);
            components.apply(delta, &scratch);
            // TODO can I unify this again?
            let brain_delta = processes.update_brain(&e, &components);
            components.apply(brain_delta, &scratch);
        });

        e.draw_3d(|stream| {
            let (vertices, indices) = scratch.render(
                camera.get_loc(), camera.get_dim(), &pixset, &components);
            let mesh = &factory.create_mesh(&vertices);
            let tri_list = indices.to_slice(factory, PrimitiveType::TriangleList).clone();
            uniforms.mvp = model_view_projection(mat4_id, camera.as_mat(), ortho_projection);
            stream.clear(clear_data);
            stream.draw(&(mesh, tri_list, &program, &uniforms, &state)).unwrap();
        });

        e.press(|button| {
            match input.press(button) {
                Output::SpawnFood(window_loc) => {
                    let loc = camera.to_game_loc(window_loc);
                    let ref mut chunk = chunks.get_chunk(&WorldCoord::from_loc(&loc).get_chunk_loc());
                    // TODO some sort of "blueprint"
                    let entity = Entity::new();
                    let color = [0.2313725, 0.3254902, 0.1372549];
                    components.new_render_component(entity, Pix::Food, color);
                    components.new_position_component(entity, loc);
                    components.new_food_component(entity, Food::Meat, 100.0);
                    chunk.insert_entity(entity); // maybe chunks.add_entity_to_chunk?
                    // TODO do bounds checking
                    scratch.insert_into_entities(entity);
                },
                Output::SpawnWall(_) => {}, // chunks.spawn_wall(camera.to_game_loc(window_loc)),
                Output::CameraMove(dir) => {
                    camera.pan(dir);
                    let camera_loc = camera.get_loc();
                    let camera_offset = camera_loc - scratch.get_loc();
                    let scratch_size = scratch.get_size();
                    let w = (scratch_size.width  - camera_dim.width)  / 2 - camera_offset.x;
                    let h = (scratch_size.height - camera_dim.height) / 2 + camera_offset.y;
                    if w.abs() == 16 || h.abs() == 16 {
                        let wc = WorldCoord::from_loc(&camera_loc).get_chunk_loc();
                        let loc = WorldCoord::from_chunk_loc(&wc).to_loc();
                        // TODO dont hardcode this
                        let scratch_dim = Loc { x: 48, y: -64 };
                        scratch = Scratch::new(loc - scratch_dim, scratch_size).inflate(&mut chunks);
                    }
                },
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
