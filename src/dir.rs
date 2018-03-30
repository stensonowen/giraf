
use std::fmt;
use std::rc::Rc;

use edge::{Edge, EdgeT};
use vertex::NodeT;

///////////////////////////////////////////////////////////////////////////////
//  Direction
///////////////////////////////////////////////////////////////////////////////

pub trait DirT<E: EdgeT>: fmt::Debug {
    fn new() -> Self;
    fn degree(&self) -> usize;
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
    fn degree(&self) -> usize { self.children.len() + self.parents.len() }
}
impl<V: NodeT, E: EdgeT> DirT<E> for Undir<V,E> {
    fn new() -> Self { Undir { neighbors: vec![] } }
    fn degree(&self) -> usize { self.neighbors.len() }
}

impl<V: NodeT, E: EdgeT> Dir<V,E> {
    pub(super) fn register_child(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.children.push(e);
    }
    pub(super) fn register_parent(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.parents.push(e);
    }
    pub(super) fn get_parents(&self) -> &[Rc<Edge<V, E, Self>>] {
        &self.parents[..]
    }
    pub(super) fn get_children(&self) -> &[Rc<Edge<V, E, Self>>] {
        &self.children[..]
    }
}

impl<V: NodeT, E: EdgeT> Undir<V,E> {
    pub(super) fn register_neighbor(&mut self, e: Rc<Edge<V, E, Self>>) {
        self.neighbors.push(e);
    }
    pub(super) fn get_neighbors(&self) -> &[Rc<Edge<V, E, Self>>] {
        &self.neighbors[..]
    }
}
