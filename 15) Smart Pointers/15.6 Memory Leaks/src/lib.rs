use std::{cell::RefCell, rc::{Rc, Weak}};
use crate::List::{Cons, Nil};

// region: for reference cycle demo

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, list) => Some(list),
            Nil => None,
        }
    }
}

// endregion: for reference cycle demo

// region: for Weak<T> demo

#[derive(Debug)]
pub struct Node {
    pub value: i32,

    /*
    Each node should own its children, but be able to share that ownership so
    that other variables can access them, so we wrap Node in an Rc<T> (Rc<Node>).
    
    We wrap the list of nodes in a RefCell<T> (RefCell<Vec<Rc<Node>>>)
    so that we can modify which nodes are children.
    */
    pub children: RefCell<Vec<Rc<Node>>>,

    /*
    We want children to reference their parents, but not own them, so Rc<T> isn't appropriate.

    Also, using an Rc<T> would create a reference cycle, since the child Node would Rc the
    parent Node, and the parent Node would Rc the child Node. Both Rc's strong_count would never be 0.

    This is the case for a weak reference!
    */
    pub parent: RefCell<Weak<Node>>,
}

// endregion: for Weak<T> demo