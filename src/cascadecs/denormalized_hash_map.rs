
use std::collections::HashMap;
use std::collections::hash_map;

use loc::Loc;
use cascadecs::entity::Entity;
use cascadecs::position_component::PositionComponent;

// TODO make this generic
pub struct DenormalizedHashMap {
    hm: HashMap<Entity, PositionComponent>,
    rhm: HashMap<Loc, Entity>,
}

impl DenormalizedHashMap {
    pub fn new() -> Self {
        DenormalizedHashMap { hm: HashMap::new(), rhm: HashMap::new() }
    }

    pub fn insert(&mut self, entity: Entity, pc: PositionComponent) -> Option<PositionComponent> {
        self.rhm.insert(pc.loc, entity);
        self.hm.insert(entity, pc)
    }

    pub fn remove(&mut self, entity: &Entity) -> Option<PositionComponent> {
        if let Some(pc) = self.hm.remove(entity) {
            self.rhm.remove(&pc.loc);
            return Some(pc)
        }
        None
    }

    pub fn get(&self, entity: &Entity) -> Option<&PositionComponent> {
        self.hm.get(entity)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut PositionComponent> {
        self.hm.get_mut(entity)
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> hash_map::Iter<Entity, PositionComponent> {
        self.hm.iter()
    }

    pub fn get_by_value(&self, loc: Loc) -> Option<&Entity> {
        self.rhm.get(&loc)
    }
}
