use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use loc::Loc;

pub type LocMap<T> = HashMap<Loc, Rc<RefCell<T>>>;

// http://stackoverflow.com/questions/29345708/matching-a-generic-parameter-to-an-associated-type-in-an-impl
