
extern crate snowflake;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(snowflake::ProcessUniqueId);

impl Entity {
    pub fn new() -> Entity {
        Entity(snowflake::ProcessUniqueId::new())
    }
}
