use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use loc::Loc;
use renderable::Renderable;

pub type Entities = HashMap<Loc, Rc<RefCell<Renderable>>>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EntityState {
    Ok,
    OOP,
}
