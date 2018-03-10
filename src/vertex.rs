// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use std::fmt;
use std::hash::Hash;

pub trait Node: fmt::Debug + Eq + Hash {}

#[derive(Debug, Hash, Eq)] 
pub struct Vertex<T: Node> {
    val: T,
}

impl<T: Node> Vertex<T> {
    pub(crate) fn from(val: T) -> Self {
        Vertex { val, }
    }
    //pub(crate) fn inner(&self) -> &T { self.val }
}

impl<T: Node> PartialEq<T> for Vertex<T> {
    fn eq(&self, other: &T) -> bool {
        self.val == *other
    }
}

impl<T: Node> PartialEq<Vertex<T>> for Vertex<T> {
    fn eq(&self, other: &Vertex<T>) -> bool {
        self.val == other.val
    }
}


