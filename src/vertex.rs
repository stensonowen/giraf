
use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use std::marker::PhantomData;

use dir::{DirT, Undir, Dir};
use edge::{EdgeT, Edge};

///////////////////////////////////////////////////////////////////////////////
//  Vertex
///////////////////////////////////////////////////////////////////////////////

pub trait NodeT: fmt::Debug + Eq + Hash {}
impl<T: fmt::Debug + Eq + Hash> NodeT for T {}

#[derive(Debug)]
pub(super) struct Vertex<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: Rc<V>,
    hood: D,
    _e: ::std::marker::PhantomData<E>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Vertex<V,E,D> {
    pub(crate) fn new(val: V) -> Self {
        Vertex {
            val: Rc::new(val),
            hood: D::new(),
            _e: PhantomData,
        }
    }
    pub(super) fn borrow(&self) -> Rc<V> {
        self.val.clone()
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Undir<V,E>> {
    pub(super) fn register_neighbor(&mut self, e: Rc<Edge<V, E, Undir<V,E>>>) {
        self.hood.register_neighbor(e);
    }
    /*
    pub(super) fn get_neighbors(&self) -> Vec<&Self> {
        let v: Vec<_> = self.hood.get_neighbors().into_iter()
            //.map(|ptr| ptr as &Vertex<V, E, Undir<V,E>>)
            //.map(|ptr| &*ptr)
            .map(|&ptr| {
                let b = self.borrow();
                let other = unsafe { (*ptr).get_other_endpoint(b) };
                unimplemented!()
            }).collect(); 
        v
    }
    */
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Dir<V,E>> {
    pub(super) fn register_parent(&mut self, e: Rc<Edge<V, E, Dir<V,E>>>) {
        self.hood.register_parent(e);
    }
    pub(super) fn register_child(&mut self, e: Rc<Edge<V, E, Dir<V,E>>>) {
        self.hood.register_child(e);
    }
}
