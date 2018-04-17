
use std::collections::HashSet;

use Graph;
use dir::{DirT};
use edge::{EdgeT};
use vertex::{NodeT, Vertex};

//pub type Component<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> = Vec<&'a Vertex<V,E,D>>;
pub type Component<'a, V, E, D> = Vec<&'a Vertex<V,E,D>>;

pub struct Components<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    graph: &'a Graph<V,E,D>,
    seen: HashSet<&'a V>,
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Components<'a,V,E,D> {
    pub(crate) fn new(g: &'a Graph<V,E,D>) -> Self {
        Components { graph: g, seen: HashSet::with_capacity(g.order()) }
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Iterator for Components<'a,V,E,D> {
    type Item = Component<'a,V,E,D>;
    fn next(&mut self) -> Option<Component<'a,V,E,D>> {
        let start = self.graph.vertices().find(|v| !self.seen.contains(v.as_ref()))?;
        let mut component = Vec::new();
        for reachable in self.graph.depth_first(Some(start)) {
            self.seen.insert(reachable.as_ref());
            component.push(reachable);
        }
        if component.is_empty() {
            None
        } else {
            Some(component)
        }
    }
}

