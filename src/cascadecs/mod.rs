
pub mod entity;
pub mod components;
// TODO namespace all components under components (read the modules chapter in the book)
pub mod hunger_component;
pub mod position_component;
pub mod render_component;

mod event;

mod process;
mod movement_process;
mod render_process;
