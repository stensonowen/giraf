#![allow(unused)]
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
 *  maybe Edge should only contain 'other' vertex and Rc<E> ?
 *  pretty cool to store neighbors as a HashMap<Rc<V>, Rc<E>>
 *
 *  CLEANUP
 *      consolidate ret/panic behavior (rn we ret None on bad edge insert but panic! on vert)
 *
 *  More trait-based guarantees: make more things trait methods
 *      so more implementations can be generic (e.g. Neighbor stuff)
 *      e.g. Vertex/Dir::register_as_lhs or sthg
 *
 */

use std::slice;
use std::rc::Rc;
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
pub struct Graph<V: NodeT, E: EdgeT, D: DirT<V,E>> {
    nodes: HashMap<Rc<V>, Vertex<V,E,D>>,
    // TODO can change `nodes` to a HashSet if we overload Vert::borrow
    //  is that desirable?
    // TODO is it just me or does `<V,E,D>` look like "venereal disease"?
    //edges: Vec<GenEdge<V,E,D>>,
    //edges: Vec<E>, // TODO get rid of this? no central ownership?
    edges: Vec<Rc<E>>, // TODO get rid of this? no central ownership?
}


impl<V: NodeT, E: EdgeT, D: DirT<V,E>> Graph<V,E,D> {
    // ctors
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: Vec::new() } }
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
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool 
        where Rc<V>: ::std::borrow::Borrow<Q>, Q: Eq + ::std::hash::Hash 
    {
        self.nodes.contains_key(k)
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
    pub fn get_reachable<'a>(&'a self, vert: &'a Vertex<V,E,D>) 
        -> iter::Neighbors<'a,V,E,D>
    {
        let reachable = vert.get_reachable().iter();
        iter::Neighbors::reachable(self, vert, reachable)
    }
    pub fn edges(&self) -> slice::Iter<Rc<E>> {
        // should this be a different Item? e.g. just a &'a (&V,&V)?
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
    pub fn components<'a>(&'a self) -> iter::Components<V,E,D> {
        iter::Components::new(self)
    }

    // modifiers
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        self.nodes.insert(vert.get_ref(), vert);
    }
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<&E> {
        let edge = Rc::new(e);
        self.edges.push(edge.clone());

        let lr = Edge::new(edge.clone(), self.get_vertex(r)?.get_ref());
        let rl = Edge::new(edge,         self.get_vertex(l)?.get_ref());

        self.nodes.get_mut(l)?.register_as_src(lr);
        self.nodes.get_mut(r)?.register_as_dst(rl);

        self.edges.last().map(|e| e.as_ref())
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_undirected_edge(&mut self, e: E, l: &V, r: &V) -> Option<&E> {
        self.insert_edge(e, l, r)
    }
    /*
    pub fn get_neighbors<'a>(&'a self, vert: &'a Vertex<V, E, Undir<V,E>>)
        -> iter::Neighbors<'a, V, E, Undir<V,E>>
    {
        let neighbors = vert.get_neighbor_edges().iter();
        iter::Neighbors::undir_neighbors(self, vert, neighbors)
    }
    */
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Dir<V,E>> {
    pub fn insert_directed_edge(&mut self, e: E, l: &V, r: &V) -> Option<&E> {
        self.insert_edge(e, l, r)
    }
    /*
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
    */
}




/*
impl<V: NodeT, E: EdgeT, D: DirT<V,E>> Index<V> for Graph<V,E,D> {
    type Output = ();
    fn index(&self, idx: V) -> &() {
        &()
    }
}
*/

/*
impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Index<&'a Vertex<V,E,D>> for Graph<V,E,D> {
    type Output = Vertex<V,E,D>;
    fn index(&self, idx: &'a Vertex<V,E,D>) -> &Vertex<V,E,D> {
        idx
    }
}
*/

/*
use std::ops::Index;
impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Index<&'a V> for Graph<V,E,D> {
    type Output = Vertex<V,E,D>;
    fn index(&self, idx: &V) -> &Vertex<V,E,D> {
        self.get_vertex(idx).unwrap()
    }
}

impl<'a, V: NodeT, E: EdgeT, D: DirT<V,E>> Index<&'a Vertex<V,E,D>> for Graph<V,E,D> {
    type Output = Vertex<V,E,D>;
    fn index<'b>(&'b self, idx: &'a Vertex<V,E,D>) -> &'b Vertex<V,E,D> {
        self.get_vertex(idx.get()).unwrap()
    }
}
*/
