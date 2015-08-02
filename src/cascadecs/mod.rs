
pub mod entity;
pub mod components;
pub mod processes;

// TODO namespace all components under components (read the modules chapter in the book)
pub mod brain_component;
pub mod hunger_component;
pub mod position_component;
pub mod render_component;

pub mod event;

mod process;
mod brain_process;
mod hunger_process;

mod denormalized_hash_map;
