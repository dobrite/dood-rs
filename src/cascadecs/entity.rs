
extern crate snowflake;

// conditional impl
// http://jadpole.github.io/rust/type-system/
//impl<T: Mob> Entity for T {
//    // ...
//}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(snowflake::ProcessUniqueId);

impl Entity {
    pub fn new() -> Entity {
        Entity(snowflake::ProcessUniqueId::new())
    }
}
