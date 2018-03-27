
use std::rc::Rc;
use std::collections::HashMap;

pub mod dir;
use dir::{DirT, Dir, Undir};

pub mod edge;
use edge::{EdgeT, Edge, UnweightedEdge};

pub mod vertex;
use vertex::{NodeT, Vertex};

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
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: Vec::new(), } }
    pub fn with_capacity(n: usize, m: usize) -> Self {
        Graph { nodes: HashMap::with_capacity(n), edges: Vec::with_capacity(m) }
    }
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        self.nodes.insert(vert.borrow(), vert);
    }
    fn get_vertex(&self, v: &V) -> Option<&Vertex<V,E,D>> {
        self.nodes.get(v)
    }
    fn create_edge(&mut self, e: E, l: &V, r: &V) -> Option<Rc<Edge<V,E,D>>> {
        let edge = {
            let lhs = self.get_vertex(l)?;
            let rhs = self.get_vertex(r)?;
            Edge::new(e, lhs.borrow(), rhs.borrow())
        };
        let edge = Rc::new(edge);
        self.edges.push(edge.clone());
        Some(edge)
    }
    pub fn get_edge(&self, l: &V, r: &V) -> Option<Rc<Edge<V,E,D>>> {
        let lhs = self.get_vertex(l)?;
        let rhs = self.get_vertex(r)?;
        unimplemented!()
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let edge = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.register_neighbor(edge.clone());
        self.nodes.get_mut(r)?.register_neighbor(edge);
        Some(())
    }
    fn get_neighbors(&self, vert: &V) -> Option<&Vertex<V, E, Undir<V,E>>> {
        unimplemented!()
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Dir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let edge = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.register_child(edge.clone());
        self.nodes.get_mut(r)?.register_parent(edge);
        Some(())
    }
}

