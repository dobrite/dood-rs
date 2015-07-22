use std::collections::HashMap;

use pixset::Pix;
use loc::Loc;

use cascadecs::entity::Entity;
use cascadecs::hunger_component::HungerComponent;
use cascadecs::render_component::RenderComponent;
use cascadecs::position_component::PositionComponent;

pub struct Components {
    // TODO fixme
    pub hunger_components: HashMap<Entity, HungerComponent>,
    pub render_components: HashMap<Entity, RenderComponent>,
    pub position_components: HashMap<Entity, PositionComponent>,
}

impl Components {
    pub fn new() -> Components {
        Components {
            hunger_components: HashMap::new(),
            render_components: HashMap::new(),
            position_components: HashMap::new(),
        }
    }

    pub fn new_render_component(&mut self, entity: Entity, pix: Pix, color: [f32; 3]) {
        self.render_components.insert(entity, RenderComponent::new(pix, color));
    }

    pub fn new_position_component(&mut self, entity: Entity, loc: Loc) {
        self.position_components.insert(entity, PositionComponent::new(loc));
    }
}
