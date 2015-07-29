
pub mod entity;
pub mod components;
pub mod processes;
// TODO namespace all components under components (read the modules chapter in the book)
pub mod hunger_component;
pub mod position_component;
pub mod render_component;

mod event;

mod process;
mod hunger_process;
mod movement_process;

mod denormalized_hash_map;
