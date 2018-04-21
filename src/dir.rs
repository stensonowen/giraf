
use std::fmt;

use edge::{EdgeT, GenEdge, DirEdge, UndirEdge};
use vertex::NodeT;

///////////////////////////////////////////////////////////////////////////////
//  Direction
///////////////////////////////////////////////////////////////////////////////

pub trait DirT<V: NodeT, E: EdgeT>: fmt::Debug + Sized {
    fn new() -> Self;
    fn degree(&self) -> usize;
    fn push_src(&mut self, edge: GenEdge<V, E, Self>);
    fn push_dst(&mut self, edge: GenEdge<V, E, Self>);
    fn get_reachable(&self) -> &[GenEdge<V, E, Self>];
}

#[derive(Debug)] 
pub struct Dir<V: NodeT, E: EdgeT> {
    children: Vec<DirEdge<V,E>>,
    parents: Vec<DirEdge<V,E>>,
}

#[derive(Debug)] 
pub struct Undir<V: NodeT, E: EdgeT> {
    neighbors: Vec<UndirEdge<V,E>>,
}

impl<V: NodeT, E: EdgeT> DirT<V,E> for Dir<V,E> {
    fn new() -> Self { Dir { children: vec![], parents: vec![] } }
    fn degree(&self) -> usize { self.children.len() + self.parents.len() }
    fn push_src(&mut self, edge: GenEdge<V, E, Self>) { self.parents.push(edge); }
    //fn push_dst(&mut self, _edge: GenEdge<V, E, Self>) { /*self.children.push(edge);*/ } // uhhh
    fn push_dst(&mut self, _edge: GenEdge<V, E, Self>) { self.children.push(_edge); } // uh
    fn get_reachable(&self) -> &[GenEdge<V, E, Self>] { self.get_children() }
}
impl<V: NodeT, E: EdgeT> DirT<V,E> for Undir<V,E> {
    fn new() -> Self { Undir { neighbors: vec![] } }
    fn degree(&self) -> usize { self.neighbors.len() }
    fn push_src(&mut self, edge: GenEdge<V, E, Self>) { self.neighbors.push(edge); }
    fn push_dst(&mut self, edge: GenEdge<V, E, Self>) { self.neighbors.push(edge); }
    fn get_reachable(&self) -> &[GenEdge<V, E, Self>] { self.get_neighbors() }
}

impl<V: NodeT, E: EdgeT> Dir<V,E> {
    pub(super) fn get_parents(&self) -> &[DirEdge<V,E>]  { &self.parents[..] }
    pub(super) fn get_children(&self) -> &[DirEdge<V,E>] { &self.children[..] }
}

impl<V: NodeT, E: EdgeT> Undir<V,E> {
    pub(super) fn get_neighbors(&self) -> &[UndirEdge<V,E>] {
        &self.neighbors[..]
    }
}
