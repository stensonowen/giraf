
use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use std::borrow::Borrow;
use std::marker::PhantomData;

use dir::{DirT, Undir, Dir};
use edge::{EdgeT, DirEdge, UndirEdge};

///////////////////////////////////////////////////////////////////////////////
//  Vertex
///////////////////////////////////////////////////////////////////////////////

pub trait NodeT: fmt::Debug + Eq + Hash {}
impl<T: fmt::Debug + Eq + Hash> NodeT for T {}

#[derive(Debug)]
pub struct Vertex<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: Rc<V>,
    hood: D,
    _e: PhantomData<E>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Vertex<V,E,D> {
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
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Borrow<V> for Vertex<V,E,D> {
    fn borrow(&self) -> &V {
        &self.val
    }
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> AsRef<V> for Vertex<V,E,D> {
    fn as_ref(&self) -> &V {
        &self.val
    }
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Undir<V,E>> {
    //pub(super) fn register_neighbor(&mut self, e: Rc<Edge<V, E, Undir<V,E>>>) {
    pub(super) fn register_neighbor(&mut self, e: UndirEdge<V,E>) {
        self.hood.register_neighbor(e);
    }
    pub(super) fn get_neighbor_edges(&self) -> &[UndirEdge<V,E>] {
        self.hood.get_neighbors()
    }
    /*
    pub(super) fn get_neighbors(&self) -> Vec<Rc<V>> {
        let b = &self.val;
        self.hood.get_neighbors().iter().map(|e| {
            e.get_other_endpoint(b).unwrap()
        }).collect()
    }
    pub(super) fn get_neighbors_i<'a>(&'a self) -> Box<Iterator<Item=Rc<V>> + 'a> {
        let b = &self.val;
        Box::new(self.hood.get_neighbors().iter().map(move |e| {
            e.get_other_endpoint(b).unwrap()
        }))
    }
    */
}

impl<V: NodeT, E: EdgeT> Vertex<V, E, Dir<V,E>> {
    pub(super) fn register_parent(&mut self, e: DirEdge<V,E>) {
        self.hood.register_parent(e);
    }
    pub(super) fn register_child(&mut self, e: DirEdge<V,E>) {
        self.hood.register_child(e);
    }
    /*
    pub(super) fn get_parents(&self) -> Vec<&V> {
        self.hood.get_parents().iter().map(|e| e.get_src()).collect()
    }
    pub(super) fn get_parents_i<'a>(&'a self) -> Box<Iterator<Item=&'a V>+'a> {
        Box::new(self.hood.get_parents().iter().map(|e| e.get_src()))
    }
    pub(super) fn get_children(&self) -> Vec<&V> {
        self.hood.get_children().iter().map(|e| e.get_dst()).collect()
    }
    pub(super) fn get_children_i<'a>(&'a self) -> Box<Iterator<Item=&'a V>+'a> {
        Box::new(self.hood.get_children().iter().map(|e| e.get_dst()))
    }
    */

    pub(super) fn get_parent_edges(&self) -> &[DirEdge<V,E>] {
        self.hood.get_parents()
    }
    pub(super) fn get_child_edges(&self) -> &[DirEdge<V,E>] {
        self.hood.get_children()
    }
}

