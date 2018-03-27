
use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt;

use edge::{EdgeT, Edge};
use dir::{DirT, Undir, Dir};

///////////////////////////////////////////////////////////////////////////////
//  Vertex
///////////////////////////////////////////////////////////////////////////////

pub trait NodeT: 'static + fmt::Debug + Eq + Hash {}
impl<T: 'static + fmt::Debug + Eq + Hash> NodeT for T {}

#[derive(Debug)]
pub(super) struct Vertex<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: Box<V>,
    pub hood: D,
    _e: ::std::marker::PhantomData<E>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Vertex<V,E,D> {
    pub(crate) fn new(val: V) -> Self {
        Vertex {
            val: Box::new(val),
            hood: D::new(),
            _e: PhantomData,
        }
    }
    pub(super) fn borrow(&self) -> &V {
        &self.val
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Undir<V,E>> {
    pub(super) fn register_neighbor(&mut self, e: *const Edge<V, E, Undir<V,E>>) {
        self.hood.register_neighbor(e);
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Dir<V,E>> {
    pub(super) fn register_parent(&mut self, e: *const Edge<V, E, Dir<V,E>>) {
        self.hood.register_parent(e);
    }
    pub(super) fn register_child(&mut self, e: *const Edge<V, E, Dir<V,E>>) {
        self.hood.register_child(e);
    }
}
