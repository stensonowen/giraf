
use std::rc::Rc;
use std::collections::hash_map;

use Graph;
use dir::DirT;
use edge::EdgeT;
use vertex::{NodeT, Vertex};

pub struct Vertices<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    inner: &'a Graph<V,E,D>,
    iter: hash_map::Values<'a, Rc<V>, Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> Vertices<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>) -> Self {
        Vertices {
            inner: g,
            iter: g.map_vals(),
        }
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<E>> Iterator for Vertices<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        self.iter.next()
    }
}
