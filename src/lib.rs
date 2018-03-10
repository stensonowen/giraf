#![allow(unused)]

use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

mod edge;
use edge::{Edge};
use edge::{EdgeDir, DirectedEdge, UndirectedEdge};
use edge::{EdgeWeight, SignedEdge, UnsignedEdge, UnweightedEdge};

mod vertex;
use vertex::{Vertex, Node};


#[derive(Debug)]
pub struct Graph<V: Node, D: EdgeDir<V,W>, W: EdgeWeight> {
    // `State` element: can store counter if in building state
    // maybe use `rental` to refer to self? does that mean we can't move G?
    edges: Vec<Edge<V,D,W>>,
    vertices: HashSet<Vertex<V,D,W>>,
    //nodes: HashMap<Vertex<V>, Vec<Edge<V,D,W>>>,
}

impl<V: Node, D: EdgeDir<V,W>, W: EdgeWeight> Graph<V,D,W> {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            vertices: HashSet::new(),
            //nodes: HashMap::new(),
        }
    }
    /// A vertex can be added to any graph, regarrdless of edge weighted-ness or directed-ness
    pub fn add_vertex(&mut self, val: V) -> &Vertex<V,D,W> {
        let vertex = Vertex::new(val);
        self.vertices.insert(vertex.clone());
        self.vertices.get(&vertex).unwrap()
    }

    pub fn contains_value(&self, val: &V) -> bool {
        self.vertices.contains(val)
    }
    pub fn contains_vertex(&self, val: &Vertex<V,D,W>) -> bool {
        self.vertices.contains(val)
    }
}

// ******************************************************************
// **********          Unweighted                          **********
// ******************************************************************
impl<V: Node, D: EdgeDir<V,UnweightedEdge>> Graph<V, D, UnweightedEdge> {
    // returns edge reference or None if nothing was added (invalid input)
    // inputs must have `self` lifetime, so they're valid unless deleted
    // can't just take V as input: would need to create Vertex<&V> :/
    pub fn add_edge<'a>(&'a mut self, 
                        l: &'a Vertex<V, D, UnweightedEdge>, 
                        r: &'a Vertex<V, D, UnweightedEdge>) 
                        -> Option<&'a Edge<V,D,UnweightedEdge>> {
        let lhs = self.vertices.get(l)?;
        let rhs = self.vertices.get(r)?;
        let edge = Edge::between(lhs.clone(), rhs.clone(), ());
        self.edges.push(edge);
        self.edges.last() // uhh this part should never be none
    }
}


// ******************************************************************
// **********          Undirected                          **********
// ******************************************************************
impl<V: Node, W: EdgeWeight> Graph<V, UndirectedEdge, W> {

}

// ******************************************************************
// **********          Undirected, Unweighted              **********
// ******************************************************************

impl<V: Node> Graph<V, UndirectedEdge, UnweightedEdge> {
    pub fn new_undirected_unweighted() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Directed, Unweighted                **********
// ******************************************************************
impl<V: Node> Graph<V, DirectedEdge, UnweightedEdge> {
    pub fn new_directed_unweighted() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Directed, Unsigned Weights          **********
// ******************************************************************
impl<V: Node> Graph<V, DirectedEdge, UnsignedEdge> {
    pub fn new_directed_weighted() -> Self {
        Self::new()
    }
}
// ******************************************************************
// **********          Directed, Signed Weights            **********
// ******************************************************************
impl<V: Node> Graph<V, DirectedEdge, SignedEdge> {
    pub fn new_directed_weighted_signed() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Undirected, Unsigned Weights        **********
// ******************************************************************
impl<V: Node> Graph<V, UndirectedEdge, UnsignedEdge> {
    pub fn new_undirected_weighted() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Undirected, Signed Weights          **********
// ******************************************************************
impl<V: Node> Graph<V, UndirectedEdge, SignedEdge> {
    pub fn new_undirected_weighted_signed() -> Self {
        Self::new()
    }
}


// hmmmm
//  can I use one vec of edges and only store slices of it?
//  every edge will be incident to 2 vertices (may be the same)
//  dude i don't fucken know it's too late
// todo
//  unify letters used for Vertex data (T/V/N)
