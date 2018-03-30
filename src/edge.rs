
use std::fmt;
use std::rc::Rc;
use std::borrow::Borrow;
use std::marker::PhantomData;

use dir::{DirT, Dir, Undir};
use vertex::{NodeT};

///////////////////////////////////////////////////////////////////////////////
//  Edge
///////////////////////////////////////////////////////////////////////////////

pub trait EdgeT: fmt::Debug {}
impl<T: fmt::Debug + Ord> EdgeT for T {}

#[derive(Debug)]
pub struct UnweightedEdge;
impl EdgeT for UnweightedEdge {}

pub type DirEdge<V,E>   = Rc<Edge<V, E, Dir<V,E>>>;
pub type UndirEdge<V,E> = Rc<Edge<V, E, Undir<V,E>>>;
pub type GenEdge<V,E,D> = Rc<Edge<V, E, D>>;

#[derive(Debug)]
pub struct Edge<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: E,
    _d: PhantomData<D>,
    lhs: Rc<V>,
    rhs: Rc<V>,
    // should an Edge "know" its direction? 
    //  I think so? does the need for a `PhantomData` contradict that?
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Edge<V,E,D> {
    pub(super) fn new(e: E, l: Rc<V>, r: Rc<V>) -> GenEdge<V,E,D> {
        let e = Edge { val: e, lhs: l, rhs: r, _d: PhantomData, };
        Rc::new(e)
    }
}

impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> {
    pub(super) fn get_src(&self) -> &V { &self.lhs }
    pub(super) fn get_dst(&self) -> &V { &self.rhs }
}

impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> {
    pub(super) fn get_other_endpoint(&self, t: &V) -> Option<Rc<V>> {
        match (t == (&self.lhs).borrow(), t == (&self.rhs).borrow()) {
            (false, false) => None,
            (true, false) => Some(self.rhs.clone()),
            (false, true) => Some(self.lhs.clone()),
            (true, true) => Some(self.rhs.clone()),
        }
    }
}
