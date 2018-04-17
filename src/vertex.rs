
use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use std::borrow::Borrow;
use std::marker::PhantomData;

use dir::{DirT, Undir, Dir};
use edge::{EdgeT, GenEdge, DirEdge, UndirEdge};

///////////////////////////////////////////////////////////////////////////////
//  Vertex
///////////////////////////////////////////////////////////////////////////////

pub trait NodeT: fmt::Debug + Eq + Hash {}
impl<T: fmt::Debug + Eq + Hash> NodeT for T {}

#[derive(Debug)]
pub struct Vertex<V: NodeT, E: EdgeT, D: DirT<V,E>> {
    val: Rc<V>,
    hood: D,
    _e: PhantomData<E>,
}

impl<V: NodeT, E: EdgeT, D: DirT<V,E>> Vertex<V,E,D> {
    pub(crate) fn new(val: V) -> Self {
        Vertex { val: Rc::new(val), hood: D::new(), _e: PhantomData, }
    }
    pub(super) fn get_ref(&self) -> Rc<V> {
        self.val.clone()
    }
    pub fn get(&self) -> &V {
        &self.val
    }
    pub fn degree(&self) -> usize {
        self.hood.degree()
    }

    pub(super) fn register_as_src(&mut self, edge: GenEdge<V,E,D>) {
        self.hood.push_src(edge);
    }
    pub(super) fn register_as_dst(&mut self, edge: GenEdge<V,E,D>) {
        self.hood.push_dst(edge);
    }
    pub(super) fn get_reachable(&self) -> &[GenEdge<V,E,D>] {
        self.hood.get_reachable()
    }
}

impl<V: NodeT, E: EdgeT, D: DirT<V,E>> Borrow<V> for Vertex<V,E,D> {
    fn borrow(&self) -> &V {
        &self.val
    }
}

impl<V: NodeT, E: EdgeT, D: DirT<V,E>> AsRef<V> for Vertex<V,E,D> {
    fn as_ref(&self) -> &V {
        &self.val
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Undir<V,E>> {
    pub(super) fn get_neighbor_edges(&self) -> &[UndirEdge<V,E>] {
        self.hood.get_neighbors()
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Dir<V,E>> {
    pub(super) fn get_parent_edges(&self) -> &[DirEdge<V,E>] {
        self.hood.get_parents()
    }
    pub(super) fn get_child_edges(&self) -> &[DirEdge<V,E>] {
        self.hood.get_children()
    }
}

