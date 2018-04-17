
use std::mem;
use std::rc::Rc;
use std::borrow::Borrow;
use std::collections::{hash_map, HashSet, VecDeque};

use Graph;
use dir::{DirT};
use edge::{EdgeT};
use vertex::{NodeT, Vertex};

mod neighbors;
pub use self::neighbors::Neighbors;

mod components;
pub use self::components::Components;

///////////////////////////////////////////////////////////////////////////////
// VERTICES
///////////////////////////////////////////////////////////////////////////////

pub struct Vertices<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    iter: hash_map::Values<'a, Rc<V>, Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> Vertices<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>) -> Self {
        Vertices { iter: g.map_vals() }
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Iterator for Vertices<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        self.iter.next()
    }
}

///////////////////////////////////////////////////////////////////////////////
// BREADTH-FIRST
///////////////////////////////////////////////////////////////////////////////

pub struct BreadthFirst<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    graph: &'a Graph<V,E,D>,
    seen: HashSet<&'a V>,
    this: VecDeque<&'a Vertex<V,E,D>>,
    next: VecDeque<&'a Vertex<V,E,D>>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> BreadthFirst<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>, start: Option<&'a Vertex<V,E,D>>) -> Self {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::with_capacity(g.order());
        if let Some(first) = start.or_else(|| g.vertices().next()) {
            queue.push_back(first);
            seen.insert(first.borrow());
        }
        BreadthFirst { graph: g, this: queue, next: VecDeque::new(), seen, }
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Iterator for BreadthFirst<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        if let Some(cur) = self.this.pop_front() {
            for reachable in self.graph.get_reachable(cur) {
                let val: &V = reachable.as_ref();
                if self.seen.contains(val) == false {
                    self.seen.insert(val);
                    self.next.push_back(reachable);
                }
            }
            Some(cur)
        } else if self.next.is_empty() {
            None
        } else {
            mem::swap(&mut self.this, &mut self.next);
            self.next.clear();
            self.next()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// DEPTH-FIRST
///////////////////////////////////////////////////////////////////////////////

pub struct DepthFirst<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    graph: &'a Graph<V,E,D>,
    stack: Vec<&'a Vertex<V,E,D>>,
    seen: HashSet<&'a V>,
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> DepthFirst<'a,V,E,D> {
    pub(super) fn new(g: &'a Graph<V,E,D>, start: Option<&'a Vertex<V,E,D>>) -> Self {
        let mut stack = Vec::new();
        if let Some(first) = start.or_else(|| g.vertices().next()) {
            stack.push(first);
        }
        DepthFirst { graph: g, stack, seen: HashSet::with_capacity(g.order()) }
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Iterator for DepthFirst<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        self.stack.pop().map(|next| {
            let v = next.as_ref();
            if self.seen.contains(v) == false {
                self.seen.insert(v);
                self.graph.get_reachable(next).for_each(|n| {
                    if self.seen.contains(n.as_ref()) == false {
                        self.stack.push(n);
                    }
                });
            }
            next
        })
    }
}

