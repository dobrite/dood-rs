// run all your entities independently in a functionally pure way,
// pass in a static copy of the world (from last frame) and themselves,
// and they return a new version of themselves at the end.

// mutable
fn update_entity(entity: &mut Entity, world: &World) -> Entity {
    ...
}

// immutable
fn update_entity(entity: &Entity, world: &World) -> (Vec<Event>, Vec<StateTransitions>) {
    ...
}

// example "scala" code
fn take_damage(damage: i32)(entity: Entity) -> Entity {
    entity.copy(health = clamp(e.health - damage, 0, 100));
} // partially apply with a damage value to get an "event"

// one type per possible side effect

// trait TakeDamage? "extends" from Event
fn take_damage(damage: i32, from_entity_id: EntId, to_entity_id: EntId) -> {
    // immutable version
    fn apply_to(entity: Entity) -> (Entity, Vec<Event>) {
        (entity.copy(health = entity.health - damage, 0, 100)), vec![])
    }

    // mutable version
    fn apply_to(entity: Entity) -> Vec<Event> {
        entity.health = entity.health - damage;
        vec![] // or spawn more events for other entities (or even self?)
    }
}

/*
objects contain components
objects can fire events to other game objects

objects.fireEvent maps over components and calls
  component.fireEvent

event moves through object to each component in turn, can mutate self and
  generate more events

"Invulnerability" component. First in list for takeDamage event:
  sets "TakeDamage":"Amount" = 0

FireShield: "TakeDamage" if type == Fire Amount = 0

blue rectangles are components

"begin turn" event
  RecycleSuitFiller every so many turns fire an add water event to itself,
    which fills itself up

"beging attacking"
"attacking"
"after attacking"
"reaaaallly after attacking"
-- or -- implment a priority system
"parts" get a priority.
  when you insert the part it inserts it in priority order
  allows event to iterate through in the correct order
  use priority queue
*/
