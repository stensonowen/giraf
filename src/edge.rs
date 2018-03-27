
use std::fmt;
use std::rc::Rc;
use std::marker::PhantomData;

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
    _d: PhantomData<D>,
    lhs: Rc<V>,
    rhs: Rc<V>,
    // should an Edge "know" its direction? 
    //  I think so? does the `PhantomData` contradict that?
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Edge<V,E,D> {
    pub(super) fn new(e: E, l: Rc<V>, r: Rc<V>) -> Self {
        Edge {
            val: e,
            lhs: l, rhs: r,
            _d: PhantomData,
        }
    }
    /*
    pub(super) fn get_other_endpoint(&self, t: &V) -> Option<*const Vertex<V,E,D>> {
        let (lhs, rhs) = unsafe { ((*self.lhs).borrow(), (*self.rhs).borrow()) };
        match (t == lhs, t == rhs) {
            (false, false) => None,
            (true, false) => Some(self.rhs),
            (false, true) => Some(self.rhs),
            (true, true) => Some(self.rhs),
        }
    }
    */
}

impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> { }
impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> {
    //fn get_neighbors(&self) -> &[&Vertex<V, E, Undir<V,E>>] {
    //    //let n = self.get_neighbors();
    //    unimplemented!()
    //}
}
