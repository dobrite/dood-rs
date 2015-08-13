
use std::collections::HashMap;

use dir::Dir;
use loc::Loc;
use pixset::Pix;
use brain::Brain;

use cascadecs::event::Event;
use cascadecs::entity::Entity;

use cascadecs::brain_component::BrainComponent;
use cascadecs::hunger_component::HungerComponent;
use cascadecs::render_component::RenderComponent;
use cascadecs::position_component::PositionComponent;
use cascadecs::denormalized_hash_map::DenormalizedHashMap;

pub struct Components {
    // TODO fixme
    pub brain_components: HashMap<Entity, BrainComponent>,
    pub hunger_components: HashMap<Entity, HungerComponent>,
    pub render_components: HashMap<Entity, RenderComponent>,
    pub position_components: DenormalizedHashMap,
}

impl Components {
    pub fn new() -> Components {
        Components {
            brain_components: HashMap::new(),
            hunger_components: HashMap::new(),
            render_components: HashMap::new(),
            position_components: DenormalizedHashMap::new(),
        }
    }

    pub fn apply(&mut self, events: Vec<Event>) {
        for event in events.into_iter() {
            match event {
                Event::Hunger { entity, minus_hunger } => {
                    if let Some(hc) = self.hunger_components.get_mut(&entity) {
                        hc.value -= minus_hunger as u16;
                    }
                },
                Event::Movement { entity, dir } => {
                    if let Some(pc) = self.position_components.get_mut(&entity) {
                        match dir {
                            Dir::Down => { pc.loc.y -= 1 },
                            Dir::Up   => { pc.loc.y += 1 },
                            _ => {}
                        }
                    }
                },
                Event::UpdateBrainState { entity, state } => {
                    if let Some(bc) = self.brain_components.get_mut(&entity) {
                        bc.state = state
                    }
                },
            }
        }
    }

    pub fn new_brain_component(&mut self, entity: Entity, brain: Brain) {
        self.brain_components.insert(entity, BrainComponent::new(brain));
    }

    pub fn new_render_component(&mut self, entity: Entity, pix: Pix, color: [f32; 3]) {
        self.render_components.insert(entity, RenderComponent::new(pix, color));
    }

    pub fn new_position_component(&mut self, entity: Entity, loc: Loc) {
        self.position_components.insert(entity, PositionComponent::new(loc));
    }

    pub fn new_hunger_component(&mut self, entity: Entity, initial: u16, rate: u8) {
        self.hunger_components.insert(entity, HungerComponent::new(initial, rate));
    }

    pub fn get_render_component(&self, entity: Entity) -> Option<&RenderComponent> {
        self.render_components.get(&entity)
    }

    pub fn get_position_component(&self, entity: Entity) -> Option<&PositionComponent> {
        self.position_components.get(&entity)
    }

    pub fn get_hunger_component(&self, entity: Entity) -> Option<&HungerComponent> {
        self.hunger_components.get(&entity)
    }
}
