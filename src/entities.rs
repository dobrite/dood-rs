use std::collections::HashMap;
use std::any::Any;

use loc::Loc;

pub type Entities = HashMap<Loc, Box<Any>>;
