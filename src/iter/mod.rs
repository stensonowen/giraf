
use std::rc::Rc;
use std::collections::{hash_map, HashSet, VecDeque};

use Graph;
use dir::{DirT, Undir};
use edge::{EdgeT};
use vertex::{NodeT, Vertex};

mod neighbors;
pub use self::neighbors::Neighbors;

///////////////////////////////////////////////////////////////////////////////
// VERTICES
///////////////////////////////////////////////////////////////////////////////

pub struct Vertices<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    iter: hash_map::Values<'a, Rc<V>, Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> Vertices<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>) -> Self {
        Vertices {
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

///////////////////////////////////////////////////////////////////////////////
// BREADTH-FIRST
///////////////////////////////////////////////////////////////////////////////

/*
pub struct BreadthFirst<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    graph: &'a Graph<V,E,D>,
    seen: HashSet<&'a V>,
    this: VecDeque<&'a Vertex<V,E,D>>,
    next: Vec<&'a Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> BreadthFirst<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>, start: Option<&'a Vertex<V,E,D>>) -> Self {
        let mut queue = VecDeque::new();
        if let Some(first) = start {
            queue.push_back(first);
        } else if let Some(random) = g.vertices().next() {
            queue.push_back(random);
        }
        BreadthFirst { graph: g, this: queue, next: vec![], seen: HashSet::new() }
    }
}

impl<'a, V: NodeT, E: EdgeT> Iterator for BreadthFirst<'a, V, E, Undir<V,E>> {
    type Item = &'a Vertex<V, E, Undir<V,E>>;
    fn next(&mut self) -> Option<&'a Vertex<V, E, Undir<V,E>>> {
        if let Some(cur) = self.this.pop_front() {
            //for neighbor in self.graph.get_neighbors_i(&cur) { }
            //for neighbor in cur.foo() {}

        }
        unimplemented!()
    }
}

///////////////////////////////////////////////////////////////////////////////
// DEPTH-FIRST
///////////////////////////////////////////////////////////////////////////////

pub struct DepthFirst<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    graph: &'a Graph<V,E,D>,
    seen: HashSet<&'a V>,
    //this: VecDeque<&'a Vertex<V,E,D>>,
    //next: Vec<&'a Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> BreadthFirst<'a,V,E,D> {
    /*
    pub(super) fn new(g: &'a Graph<V,E,D>, start: Option<&'a Vertex<V,E,D>>) -> Self {
        let mut queue = VecDeque::new();
        if let Some(first) = start {
            queue.push_back(first);
        } else if let Some(random) = g.vertices().next() {
            queue.push_back(random);
        }
        BreadthFirst { graph: g, this: queue, next: vec![], seen: HashSet::new() }
    }
    */
}

/*
impl<'a, V: NodeT, E: EdgeT, D: DirT<E>> Iterator for BreadthFirst<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        unimplemented!()
    }
}
*/

*/
