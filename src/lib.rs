//#![allow(unused)]
#![allow(unknown_lints, bool_comparison, new_without_default_derive)]
// `x == false` is better than `!x` and I'll fight anyone who disagrees

/* TODO
 *  Tree (should be easy, try to make it easily ↔ Graph
 *  Path/path stuff for ret vals
 *  CopyGraph: cheaper alternative when T:Copy w/ fewer allocs
 *  quickcheck tests, mutate, quickcheck alt?
 *  graph variant shortcuts (e.g. `Graph::new_digraph()`)
 *  remove vertices/edges (assert strong count == 0)
 *  graph ops
 *      depth-first-search, breadth-first-search
 *  look up vertex / edge by either &'a ref or by &V/(&V,&V) ?
 *      separate functions? or with a neat trait or something?
 *  separate UnweightedEdge from EdgeT to impl for unweighted graphs only
 *
 *  CLEANUP
 *      consolidate ret/panic behavior (rn we ret None on bad edge insert but panic! on vert)
 *
 */

use std::rc::Rc;
use std::slice;
use std::collections::{hash_map, HashMap};

mod dir;    use dir::{DirT, Dir, Undir};
mod edge;   use edge::{EdgeT, Edge, GenEdge}; pub use edge::UnweightedEdge;
mod vertex; use vertex::{NodeT, Vertex};
mod iter;

#[cfg(test)] mod test;

///////////////////////////////////////////////////////////////////////////////
//  Graph
///////////////////////////////////////////////////////////////////////////////

pub type UnweightedGraph<V,D> = Graph<V, UnweightedEdge, D>;
pub type UnweightedUndirectedGraph<V> = Graph<V, UnweightedEdge, Undir<V, UnweightedEdge>>;
pub type UndirectedGraph<V,E> = Graph<V, E, Undir<V,E>>;
pub type DiGraph<V,E> = Graph<V, E, Dir<V,E>>;


#[derive(Debug)]
pub struct Graph<V: NodeT, E: EdgeT, D: DirT<E>> {
    nodes: HashMap<Rc<V>, Vertex<V,E,D>>,
    // TODO can change `nodes` to a HashSet if we overload Vert::borrow
    //  is that desirable?
    // TODO is it just me or does `<V,E,D>` look like "venereal disease"?
    //edges: Vec<Rc<Edge<V,E,D>>>,
    edges: Vec<GenEdge<V,E,D>>,
}


impl<V: NodeT, E: EdgeT, D: DirT<E>> Graph<V,E,D> {
    // ctors
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: Vec::new(), } }
    pub fn with_capacity(n: usize, m: usize) -> Self {
        Graph { nodes: HashMap::with_capacity(n), edges: Vec::with_capacity(m) }
    }

    // accessors
    pub fn size(&self) -> usize { 
        self.edges.len()
    }
    pub fn order(&self) -> usize {
        self.nodes.len()
    }
    fn get_vertex(&self, v: &V) -> Option<&Vertex<V,E,D>> {
        self.nodes.get(v)
    }
    //pub fn get_vertex_val(&self, v: &V) -> Option<&V> 
    //pub fn get_edge_val(&self, l: &V, r: &V) -> Option<&E> 
    /*
    fn get_edge(&self, l: &V, r: &V) -> Option<Rc<Edge<V,E,D>>> {
        let lhs = self.get_vertex(l)?;
        let rhs = self.get_vertex(r)?;
        unimplemented!()
    }
    */

    // iterators
    fn map_vals(&self) -> hash_map::Values<Rc<V>, Vertex<V,E,D>> {
        self.nodes.values()
    }
    pub fn vertices(&self) -> iter::Vertices<V,E,D> {
        iter::Vertices::new(self)
    }
    pub fn edges(&self) -> slice::Iter<GenEdge<V,E,D>> {
        self.edges.iter()
    }
    pub fn breadth_first<'a>(&'a self, start: Option<&'a Vertex<V,E,D>>) 
        -> iter::BreadthFirst<V,E,D> 
    {
        iter::BreadthFirst::new(self, start)
    }
    pub fn depth_first<'a>(&'a self, start: Option<&'a Vertex<V,E,D>>)
        -> iter::DepthFirst<V,E,D>
    {
        iter::DepthFirst::new(self, start)
    }

    // modifiers
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        self.nodes.insert(vert.get_ref(), vert);
    }
    fn create_edge(&mut self, e: E, l: &V, r: &V) -> Option<GenEdge<V,E,D>> {
        let edge = {
            let lhs = self.get_vertex(l)?;
            let rhs = self.get_vertex(r)?;
            Edge::new(e, lhs.get_ref(), rhs.get_ref())
        };
        self.edges.push(edge.clone());
        Some(edge)
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_undirected_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let edge = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.register_neighbor(edge.clone());
        self.nodes.get_mut(r)?.register_neighbor(edge);
        Some(())
    }
    pub fn get_neighbors<'a>(&'a self, vert: &'a Vertex<V, E, Undir<V,E>>)
        -> iter::Neighbors<'a, V, E, Undir<V,E>>
    {
        let neighbors = vert.get_neighbor_edges().iter();
        iter::Neighbors::undir_neighbors(self, vert, neighbors)
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Dir<V,E>> {
    pub fn insert_directed_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let edge = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.register_child(edge.clone());
        self.nodes.get_mut(r)?.register_parent(edge);
        Some(())
    }
    pub fn get_neighbors<'a>(&'a self, vert: &'a Vertex<V, E, Dir<V,E>>) 
        -> iter::Neighbors<'a, V, E, Dir<V,E>>
    {
        let parents = vert.get_parent_edges().iter();
        let children = vert.get_child_edges().iter();
        iter::Neighbors::dir_neighbors(self, vert, parents, children)
    }
    pub fn get_parents<'a>(&'a self, vert: &'a Vertex<V, E, Dir<V,E>>) 
        -> iter::Neighbors<'a, V, E, Dir<V,E>>
    {
        let parents = vert.get_parent_edges().iter();
        iter::Neighbors::parents(self, vert, parents)
    }
    pub fn get_children<'a>(&'a self, vert: &'a Vertex<V, E, Dir<V,E>>) 
        -> iter::Neighbors<'a, V, E, Dir<V,E>>
    {
        let children = vert.get_child_edges().iter();
        iter::Neighbors::children(self, vert, children)
    }
}

