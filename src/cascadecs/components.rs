
use std::collections::HashMap;

use render::Scratch;

use dir::Dir;
use loc::Loc;
use food::Food;
use size::Size;
use pixset::Pix;
use brain::Brain;
use path::{path, Path, PathTarget};

use utils::get_closest;

use super::event::Event;
use super::entity::Entity;

use super::brain_component::BrainComponent;
use super::hunger_component::HungerComponent;
use super::render_component::RenderComponent;
use super::position_component::PositionComponent;
use super::path_component::PathComponent;
use super::food_component::FoodComponent;
use super::fov_component::FovComponent;
use super::impassable_component::ImpassableComponent;
use super::opaque_component::OpaqueComponent;

use super::denormalized_hash_map::DenormalizedHashMap;

pub struct Components {
    // TODO fixme
    pub brain_components: HashMap<Entity, BrainComponent>,
    pub hunger_components: HashMap<Entity, HungerComponent>,
    pub render_components: HashMap<Entity, RenderComponent>,
    pub path_components: HashMap<Entity, PathComponent>,
    pub food_components: HashMap<Entity, FoodComponent>,
    pub fov_components: HashMap<Entity, FovComponent>,
    pub impassable_components: HashMap<Entity, ImpassableComponent>,
    pub opaque_components: HashMap<Entity, OpaqueComponent>,
    pub position_components: DenormalizedHashMap,
}

impl Components {
    pub fn new() -> Components {
        Components {
            brain_components: HashMap::new(),
            hunger_components: HashMap::new(),
            render_components: HashMap::new(),
            path_components: HashMap::new(),
            food_components: HashMap::new(),
            fov_components: HashMap::new(),
            impassable_components: HashMap::new(),
            opaque_components: HashMap::new(),
            position_components: DenormalizedHashMap::new(),
        }
    }

    pub fn apply(&mut self, events: Vec<Event>, mut scratch: &mut Scratch, clear: bool) {
        if clear { scratch.clear_fov(); } // TODO soo gross

        for event in events.into_iter() {
            match event {
                Event::ComputeFov { entity } => {
                    let loc = match self.position_components.get(&entity) {
                        None => return,
                        Some(pc) => pc.loc
                    };

                    if let Some(fc) = self.fov_components.get_mut(&entity) {
                        let indices = scratch.loc_to_indices(loc);
                        fc.fov.compute_fov(indices.col, indices.row, fc.range, false, scratch.get_flags());
                    }
                }
                Event::Hunger { entity, minus_hunger } => {
                    if let Some(hc) = self.hunger_components.get_mut(&entity) {
                        hc.value -= minus_hunger as u16;
                    }
                },
                Event::Movement { entity, dir } => {
                    if let Some(pc) = self.position_components.get_mut(&entity) {
                        match dir {
                            Dir::Down  => { pc.loc.y -= 1 },
                            Dir::Up    => { pc.loc.y += 1 },
                            Dir::Left  => { pc.loc.x -= 1 },
                            Dir::Right => { pc.loc.x += 1 },
                        }
                    }
                },
                Event::UpdateBrainState { entity, state } => {
                    if let Some(bc) = self.brain_components.get_mut(&entity) {
                        bc.state = state
                    }
                },
                Event::PathToFood { entity } => {
                    let loc = match self.position_components.get(&entity) {
                        None => return,
                        Some(pc) => pc.loc
                    };

                    let mut hm = HashMap::new();

                    for entity in self.food_components.keys() {
                        hm.insert(self.position_components.get(&entity).unwrap().loc, *entity);
                    }

                    if let Some(goal) = get_closest(loc, hm.keys().collect::<Vec<_>>()) {
                        let path = path(scratch.get_grid(), loc, goal);
                        let target = *hm.get(&goal).unwrap();
                        self.new_path_component(entity, path, PathTarget::Entity(target));
                        self.brain_components.get_mut(&entity).unwrap().target = Some(target);
                    }
                },
                Event::PopPath { entity } => {
                    let loc_opt = {
                        let mut pc = self.path_components.get_mut(&entity).unwrap();
                        pc.path.pop()
                    };

                    if let Some(loc) = loc_opt {
                        if let Some(pc) = self.position_components.get_mut(&entity) {
                            pc.loc = loc;
                        };
                    } else {
                        self.path_components.remove(&entity);
                        return
                    };
                },
                Event::EatFood { entity, target } => {
                    let dood_loc = match self.position_components.get_mut(&entity) {
                        None => return,
                        Some(pc) => pc.loc,
                    };

                    let food_loc = match self.position_components.get_mut(&target) {
                        None => return,
                        Some(pc) => pc.loc,
                    };

                    assert_eq!(food_loc, dood_loc);

                    let (done, ate) = match self.food_components.get_mut(&target) {
                        None => return,
                        Some(fc) => {
                            let usage = fc.noms.min(20.0);
                            fc.noms -= usage;
                            (fc.noms == 0.0, usage)
                        }
                    };

                    match self.hunger_components.get_mut(&entity) {
                        None => return,
                        Some(hc) => hc.value += ate as u16
                    }

                    if done {
                        self.remove_entity(target);
                        match self.brain_components.get_mut(&entity) {
                            Some(bc) => bc.target = None,
                            _ => unreachable!(),
                        };
                    };
                },
                Event::None => {},
            }
        }
    }

    // New

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

    pub fn new_food_component(&mut self, entity: Entity, kind: Food, noms: f32) {
        self.food_components.insert(entity, FoodComponent::new(kind, noms));
    }

    pub fn new_path_component(&mut self, entity: Entity, path: Path, target: PathTarget) {
        self.path_components.insert(entity, PathComponent::new(path, target));
    }

    pub fn new_fov_component(&mut self, entity: Entity, size: Size, range: i32) {
        self.fov_components.insert(entity, FovComponent::new(size, range));
    }

    pub fn new_impassable_component(&mut self, entity: Entity) {
        self.impassable_components.insert(entity, ImpassableComponent::new());
    }

    pub fn new_opaque_component(&mut self, entity: Entity) {
        self.opaque_components.insert(entity, OpaqueComponent::new());
    }

    // GET

    pub fn get_render_component(&self, entity: Entity) -> Option<&RenderComponent> {
        self.render_components.get(&entity)
    }

    pub fn get_position_component(&self, entity: Entity) -> Option<&PositionComponent> {
        self.position_components.get(&entity)
    }

    pub fn get_hunger_component(&self, entity: Entity) -> Option<&HungerComponent> {
        self.hunger_components.get(&entity)
    }

    pub fn get_fov_component(&self, entity: Entity) -> Option<&FovComponent> {
        self.fov_components.get(&entity)
    }

    pub fn get_impassable_component(&self, entity: Entity) -> Option<&ImpassableComponent> {
        self.impassable_components.get(&entity)
    }

    pub fn get_opaque_component(&self, entity: Entity) -> Option<&OpaqueComponent> {
        self.opaque_components.get(&entity)
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.brain_components.remove(&entity);
        self.hunger_components.remove(&entity);
        self.render_components.remove(&entity);
        self.path_components.remove(&entity);
        self.food_components.remove(&entity);
        self.position_components.remove(&entity);
    }
}
