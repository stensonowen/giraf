
use std::fmt;

use vertex::{NodeT, Vertex};
use dir::{DirT, Dir, Undir};

///////////////////////////////////////////////////////////////////////////////
//  Edge
///////////////////////////////////////////////////////////////////////////////

pub trait EdgeT: fmt::Debug {}
impl<T: fmt::Debug> EdgeT for T {}

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
}
impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> { }
impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> { }
