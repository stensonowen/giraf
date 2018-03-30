
use std::slice::{self, Iter};
use std::borrow::Borrow;

use Graph;
use dir::{DirT, Undir, Dir};
use edge::{EdgeT, GenEdge, DirEdge, UndirEdge};
use vertex::{NodeT, Vertex};

///////////////////////////////////////////////////////////////////////////////
// NEIGHBORS
///////////////////////////////////////////////////////////////////////////////

// TODO: it would be mildly cool to do this with traits instead of switching, but 
//  that's like a dozen objects just to avoid one heap allocated iter, and it would
//  involve adding an AT to `DirT` just to assist in an iterator
enum NSet<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    // undirected
    Neighbors(Iter<'a, GenEdge<V,E,D>>),
    // directed
    Parents(Iter<'a, GenEdge<V,E,D>>),
    Children(Iter<'a, GenEdge<V,E,D>>),
    Both {
        parents: Iter<'a, GenEdge<V,E,D>>,
        children: Iter<'a, GenEdge<V,E,D>>,
    }
}

pub struct Neighbors<'a, V: 'a+NodeT, E: 'a+EdgeT, D: 'a+DirT<E>> {
    graph: &'a Graph<V,E,D>,
    from: &'a Vertex<V,E,D>,
    kind: NSet<'a,V,E,D>,
}


impl<'a, V: NodeT, E: EdgeT> Iterator for Neighbors<'a, V, E, Dir<V,E>> {
    type Item = &'a Vertex<V, E, Dir<V,E>>;
    fn next(&mut self) -> Option<&'a Vertex<V, E, Dir<V,E>>> {
        let other_v: Option<&'a V> = match self.kind {
            NSet::Neighbors(..) => unreachable!(),
            NSet::Parents(ref mut p) => p.next().map(|e| e.get_src()),
            NSet::Children(ref mut c) => c.next().map(|e| e.get_dst()),
            NSet::Both { ref mut parents, ref mut children } => {
                parents.next().map(|e| e.get_src())
                    .or_else(|| children.next().map(|e| e.get_dst()))
            },
        };
        other_v.and_then(|v| self.graph.get_vertex(v))
    }
}

impl<'a, V: NodeT, E: EdgeT> Iterator for Neighbors<'a, V, E, Undir<V,E>> {
    type Item = &'a Vertex<V, E, Undir<V,E>>;
    fn next(&mut self) -> Option<&'a Vertex<V, E, Undir<V,E>>> {
        let edge: Option<&'a UndirEdge<V,E>> = match self.kind {
            NSet::Neighbors(ref mut n) => n.next(),
            _ => unreachable!(),
        };
        edge.and_then(|e| e.get_other_endpoint(self.from.borrow()))
            .and_then(|v| self.graph.get_vertex(&v))
    }
}



impl<'a, V: 'a+NodeT, E: 'a+EdgeT> Neighbors<'a, V, E, Undir<V,E>> {
    pub(crate) fn undir_neighbors(g: &'a Graph<V, E, Undir<V,E>>, 
                            f: &'a Vertex<V, E, Undir<V,E>>, 
                            i: slice::Iter<'a, UndirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, from: f, kind: NSet::Neighbors(i) }
    }
}

impl<'a, V: 'a+NodeT, E: 'a+EdgeT> Neighbors<'a, V, E, Dir<V,E>> {
    pub(crate) fn dir_neighbors(g: &'a Graph<V, E, Dir<V,E>>, 
                            f: &'a Vertex<V, E, Dir<V,E>>, 
                            p: slice::Iter<'a, DirEdge<V,E>>,
                            c: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { 
            graph: g, from: f, 
            kind: NSet::Both { parents: p, children: c },
        }
    }
    pub(crate) fn parents(g: &'a Graph<V, E, Dir<V,E>>, 
                            f: &'a Vertex<V, E, Dir<V,E>>, 
                            p: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, from: f, kind: NSet::Parents(p) }
    }
    pub(crate) fn children(g: &'a Graph<V, E, Dir<V,E>>, 
                            f: &'a Vertex<V, E, Dir<V,E>>, 
                            c: slice::Iter<'a, DirEdge<V,E>>)
        -> Self
    {
        Neighbors { graph: g, from: f, kind: NSet::Children(c) }
    }
}

