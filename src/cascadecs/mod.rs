
pub mod entity;
pub mod components;
pub mod processes;

// TODO move these into app code -- they don't belong in the "lib"
pub mod brain_component;
pub mod food_component;
pub mod fov_component;
pub mod hunger_component;
pub mod impassable_component;
pub mod opaque_component;
pub mod path_component;
pub mod position_component;
pub mod render_component;

pub mod event;

mod process;
mod brain_process;
mod hunger_process;
mod fov_process;

mod denormalized_hash_map;
