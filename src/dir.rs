
use std::fmt;
use std::rc::Rc;

use edge::{Edge, EdgeT};
use vertex::NodeT;

///////////////////////////////////////////////////////////////////////////////
//  Direction
///////////////////////////////////////////////////////////////////////////////

pub trait DirT<E: EdgeT>: fmt::Debug {
    fn new() -> Self;
}

#[derive(Debug)] 
pub struct Dir<V: NodeT, E: EdgeT> {
    children: Vec<Rc<Edge<V, E, Dir<V,E>>>>,
    parents: Vec<Rc<Edge<V, E, Dir<V,E>>>>,
}

#[derive(Debug)] 
pub struct Undir<V: NodeT, E: EdgeT> {
    neighbors: Vec<Rc<Edge<V, E, Undir<V,E>>>>,
}

impl<V: NodeT, E: EdgeT> DirT<E> for Dir<V,E> {
    fn new() -> Self { Dir { children: vec![], parents: vec![] } }
}
impl<V: NodeT, E: EdgeT> DirT<E> for Undir<V,E> {
    fn new() -> Self { Undir { neighbors: vec![] } }
}

impl<V: NodeT, E: EdgeT> Dir<V,E> {
    pub(super) fn register_child(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.children.push(e);
    }
    pub(super) fn register_parent(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.parents.push(e);
    }
}

impl<V: NodeT, E: EdgeT> Undir<V,E> {
    pub(super) fn register_neighbor(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.neighbors.push(e);
    }
    //pub(super) fn get_neighbors(&self) -> &[*const Edge<V, E, Self>] {
    //    &self.neighbors[..]
    //}
}