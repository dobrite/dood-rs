extern crate snowflake;

pub struct Entity(snowflake::ProcessUniqueId);

impl Entity {
    pub fn new() -> Entity {
        Entity(snowflake::ProcessUniqueId::new())
    }
}
