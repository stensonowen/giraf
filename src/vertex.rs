// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use std::fmt;
use std::rc::Rc;
use std::hash::Hash;

pub trait Node: fmt::Debug + Eq + Hash {}

#[derive(Debug, Hash, PartialEq, Eq)] 
//pub struct Vertex<T: Node> { val: T, }
pub struct Vertex<T: Node>(Rc<T>);

impl<T: Node> Vertex<T> {
    //pub(crate) fn from(val: T) -> Self { Vertex { val, } }
    pub(crate) fn from(val: T) -> Self { Vertex(Rc::new(val)) }
    //pub(crate) fn inner(&self) -> &T { self.val }
}

/*
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
*/

use std::borrow::Borrow;
impl<T: Node> Borrow<T> for Vertex<T> {
    fn borrow(&self) -> &T {
        &self.0
    }

}

impl<T: Node> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex(self.0.clone())
    }
}

