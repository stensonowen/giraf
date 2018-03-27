
use std::fmt;

use dir::{DirT, Dir, Undir};
use vertex::{NodeT, Vertex};

///////////////////////////////////////////////////////////////////////////////
//  Edge
///////////////////////////////////////////////////////////////////////////////

pub trait EdgeT: fmt::Debug {}
impl<T: fmt::Debug + Ord> EdgeT for T {}

#[derive(Debug)]
pub struct UnweightedEdge;
impl EdgeT for UnweightedEdge {}

#[derive(Debug)]
pub(super) struct Edge<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: E,
    lhs: *const Vertex<V,E,D>,
    rhs: *const Vertex<V,E,D>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Edge<V,E,D> {
    pub(super) fn new(e: E, l: &Vertex<V,E,D>, r: &Vertex<V,E,D>) -> Self {
        Edge {
            val: e,
            lhs: l as *const Vertex<V,E,D>,
            rhs: r as *const Vertex<V,E,D>,
        }
    }
    pub(super) fn get_other_endpoint(&self, t: &V) -> Option<*const Vertex<V,E,D>> {
        let (lhs, rhs) = unsafe { ((*self.lhs).borrow(), (*self.rhs).borrow()) };
        match (t == lhs, t == rhs) {
            (false, false) => None,
            (true, false) => Some(self.rhs),
            (false, true) => Some(self.rhs),
            (true, true) => Some(self.rhs),
        }
    }
}

impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> { }
impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> {
    //fn get_neighbors(&self) -> &[&Vertex<V, E, Undir<V,E>>] {
    //    //let n = self.get_neighbors();
    //    unimplemented!()
    //}
}
