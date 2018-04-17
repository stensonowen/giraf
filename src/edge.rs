
use std::fmt;
use std::rc::Rc;
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

pub type DirEdge<V,E>   = Edge<V, E, Dir<V,E>>;
pub type UndirEdge<V,E> = Edge<V, E, Undir<V,E>>;
pub type GenEdge<V,E,D> = Edge<V, E, D>;

#[derive(Debug)]
pub struct Edge<V: NodeT, E: EdgeT, D: DirT<V,E>> {
    val: Rc<E>,
    end: Rc<V>,
    _d: PhantomData<D>,

    // should an Edge "know" its direction? 
    //  I think so? does the need for a `PhantomData` contradict that?
}

impl<V: NodeT, E: EdgeT, D: DirT<V,E>> Edge<V,E,D> {
    pub fn new(val: Rc<E>, end: Rc<V>) -> Self {
        Edge { val, end, _d: PhantomData }
    }
    pub fn get_end(&self) -> &V {
        &self.end
    }
}

/*
impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> {
    //pub(super) fn get_src(&self) -> &V { &self.lhs }
    //pub(super) fn get_dst(&self) -> &V { &self.rhs }
}

impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> {
    /*
    pub(super) fn get_other_endpoint(&self, t: &V) -> Option<Rc<V>> {
        match (t == (&self.lhs).borrow(), t == (&self.rhs).borrow()) {
            (false, false) => None,
            (true, false) => Some(self.rhs.clone()),
            (false, true) => Some(self.lhs.clone()),
            (true, true) => Some(self.rhs.clone()),
        }
    }
    */
}
*/
