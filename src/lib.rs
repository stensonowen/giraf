
/* TODO
 *  Tree (should be easy, try to make it easily ↔ Graph
 *  Path/path stuff for ret vals
 *  Use custom iterators instead of hacks
 *      breadth-, depth-first traversals? pre-/in-/post-order?
 *  CopyGraph: cheaper alternative when T:Copy w/ fewer allocs
 *  quickcheck tests, mutate, quickcheck alt?
 *  graph variant shortcuts (e.g. `Graph::new_digraph()`)
 *  remove vertices/edges (assert strong count == 0)
 *  graph ops
 *      depth-first-search, breadth-first-search
 */

use std::rc::Rc;
use std::slice::Iter;
use std::collections::{hash_map, HashMap};

mod dir;    use dir::{DirT, Dir, Undir};
mod edge;   use edge::{EdgeT, Edge};
mod vertex; use vertex::{NodeT, Vertex};
mod iter;   use iter::{Vertices};

//mod test;

///////////////////////////////////////////////////////////////////////////////
//  Graph
///////////////////////////////////////////////////////////////////////////////

//pub type UnweightedGraph<V, D: DirT<UnweightedEdge>> = Graph<V, UnweightedEdge, D>;
//pub type DiGraph<V, E> = Graph<V, E, Dir<V,E>>;


#[derive(Debug)]
pub struct Graph<V: NodeT, E: EdgeT, D: DirT<E>> {
    nodes: HashMap<Rc<V>, Vertex<V,E,D>>,
    // TODO can change `nodes` to a HashSet if we overload Vert::borrow
    //  is that desirable?
    // TODO is it just me or does `<V,E,D>` look like "venereal disease"?
    edges: Vec<Rc<Edge<V,E,D>>>,
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
    pub fn vertices(&self) -> Vertices<V,E,D> {
        Vertices::new(self)
    }
    pub fn edges(&self) -> Iter<Rc<Edge<V,E,D>>> {
        self.edges.iter()
    }

    // modifiers
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        self.nodes.insert(vert.get_ref(), vert);
    }
    fn create_edge(&mut self, e: E, l: &V, r: &V) -> Option<Rc<Edge<V,E,D>>> {
        let edge = {
            let lhs = self.get_vertex(l)?;
            let rhs = self.get_vertex(r)?;
            Edge::new(e, lhs.get_ref(), rhs.get_ref())
        };
        let edge = Rc::new(edge);
        self.edges.push(edge.clone());
        Some(edge)
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
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
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
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

