

use std::mem;
use std::collections::HashMap;

pub mod dir;
use dir::{DirT, Dir, Undir};

pub mod edge;
use edge::{EdgeT, Edge};

pub mod vertex;
use vertex::{NodeT, Vertex};

///////////////////////////////////////////////////////////////////////////////
//  Graph
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Graph<V: NodeT, E: EdgeT, D: DirT<E>> {
    nodes: HashMap<&'static V, Vertex<V,E,D>>,
    edges: Vec<Box<Edge<V,E,D>>>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Graph<V,E,D> {
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: Vec::new(), } }
    pub fn with_capacity(n: usize, m: usize) -> Self {
        Graph { nodes: HashMap::with_capacity(n), edges: Vec::with_capacity(m) }
    }
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        let val = unsafe {
            mem::transmute::<&V, &'static V>(vert.borrow())
        };
        self.nodes.insert(val, vert);
    }
    fn get_vertex(&self, v: &V) -> Option<&Vertex<V,E,D>> {
        self.nodes.get(v)
    }
    fn create_edge(&mut self, e: E, l: &V, r: &V) -> Option<*const Edge<V,E,D>> {
        let edge = {
            let lhs = self.get_vertex(l)?;
            let rhs = self.get_vertex(r)?;
            Edge::new(e, lhs, rhs)
        };
        self.edges.push(Box::new(edge));
        let edge: &Edge<V,E,D> = &self.edges.last().unwrap();
        Some(edge as *const Edge<V,E,D>)
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let ptr = self.create_edge(e, l, r)?;
        //self.nodes.get_mut(l)?.hood.neighbors.push(ptr);
        self.nodes.get_mut(l)?.register_neighbor(ptr);
        self.nodes.get_mut(r)?.register_neighbor(ptr);
        //self.nodes.get_mut(r)?.hood.neighbors.push(ptr);
        Some(())
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Dir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let ptr = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.register_child(ptr);
        self.nodes.get_mut(r)?.register_parent(ptr);
        Some(())
    }
}

