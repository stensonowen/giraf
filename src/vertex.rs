// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use edge::EdgeDirection;
use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use std::borrow::Borrow;

pub trait Node: fmt::Debug + Eq + Hash {}

#[derive(Debug, Hash, PartialEq, Eq)] 
pub struct VertexInner<T: Node, D: EdgeDirection> {
    val: T,
    d: D,
}


#[derive(Debug, Hash, PartialEq, Eq)] 
pub struct Vertex<T: Node>(Rc<T>); 

impl<T: Node> Vertex<T> {
    pub(crate) fn new(val: T) -> Self { 
        Vertex(Rc::new(val))
        //Vertex { val: Rc::new(val), }
    }
}

impl<T: Node> Borrow<T> for Vertex<T> {
    fn borrow(&self) -> &T {
        //&self.val
        &self.0
    }

}

impl<T: Node> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex(self.0.clone())
        //Vertex { val: self.val.clone() }
    }
}

