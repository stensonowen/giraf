
use std::slice::{self, Iter};

use Graph;
use dir::{DirT, Undir, Dir};
use edge::{Edge, EdgeT, GenEdge, DirEdge, UndirEdge};
use vertex::{NodeT, Vertex};

///////////////////////////////////////////////////////////////////////////////
// NEIGHBORS
///////////////////////////////////////////////////////////////////////////////

// TODO: it would be mildly cool to do this with traits instead of switching, but 
//  that's like a dozen objects just to avoid one heap allocated iter, and it would
//  involve adding an AT to `DirT` just to assist in an iterator
enum NSet<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    // either
    Reachable(Iter<'a, GenEdge<V,E,D>>),
    // undirected
    Neighbors(Iter<'a, UndirEdge<V,E>>),
    // directed
    Parents(Iter<'a, DirEdge<V,E>>),
    Children(Iter<'a, DirEdge<V,E>>),
    Both {
        parents: Iter<'a, DirEdge<V,E>>,
        children: Iter<'a, DirEdge<V,E>>,
    }
}

pub struct Neighbors<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> {
    graph: &'a Graph<V,E,D>,
    kind: NSet<'a,V,E,D>,
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Iterator for Neighbors<'a,V,E,D> {
    type Item = &'a Vertex<V,E,D>;
    fn next(&mut self) -> Option<&'a Vertex<V,E,D>> {
        let other_v: Option<&'a V> = match self.kind {
            NSet::Reachable(ref mut r) => r.next().map(Edge::get_end),
            NSet::Neighbors(ref mut n) => n.next().map(Edge::get_end),
            | NSet::Parents( ref mut d) 
            | NSet::Children(ref mut d) => d.next().map(Edge::get_end),
            NSet::Both { ref mut parents, ref mut children } => 
                parents.next().or_else(|| children.next()).map(Edge::get_end),
        };
        other_v.and_then(|v| self.graph.get_vertex(v))
    }
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<V,E>> Neighbors<'a,V,E,D> {
    pub(crate) fn reachable(g: &'a Graph<V,E,D>, 
                            r: slice::Iter<'a, GenEdge<V,E,D>>)
        -> Self
    {
        Neighbors { graph: g, kind: NSet::Reachable(r) }
    }
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT> Neighbors<'a, V, E, Undir<V,E>> {
    pub(crate) fn undir_neighbors(g: &'a Graph<V, E, Undir<V,E>>, 
                                  i: slice::Iter<'a, UndirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, kind: NSet::Neighbors(i) }
    }
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT> Neighbors<'a, V, E, Dir<V,E>> {
    pub(crate) fn dir_neighbors(g: &'a Graph<V, E, Dir<V,E>>, 
                                p: slice::Iter<'a, DirEdge<V,E>>,
                                c: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, kind: NSet::Both { parents: p, children: c } }
    }
    pub(crate) fn parents(g: &'a Graph<V, E, Dir<V,E>>, 
                          p: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, kind: NSet::Parents(p) }
    }
    pub(crate) fn children(g: &'a Graph<V, E, Dir<V,E>>, 
                           c: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, kind: NSet::Children(c) }
    }
}

