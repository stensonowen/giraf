#![allow(unused)]

use std::fmt;
use std::collections::HashSet;

mod edge;
use edge::{Edge};
use edge::{EdgeDirection, DirectedEdge, UndirectedEdge};
use edge::{EdgeWeight, WeightedEdge, UnweightedEdge};

mod vertex;
use vertex::{Vertex, Node};


#[derive(Debug)]
pub struct Graph<V: Node, D: EdgeDirection, W: EdgeWeight> {
    // `State` element: can store counter if in building state
    // maybe use `rental` to refer to self? does that mean we can't move G?
    edges: Vec<Edge<V,D,W>>,
    //vertices: HashSet<Rc<Vertex<V>>>,
    vertices: HashSet<Vertex<V>>,
}

impl<V: Node, D: EdgeDirection, W: EdgeWeight> Graph<V,D,W> {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            vertices: HashSet::new(),
        }
    }
    /// A vertex can be added to any graph, regarrdless of edge weighted-ness or directed-ness
    pub fn add_vertex(&mut self, val: V) -> &Vertex<V> {
        let vertex = Vertex::from(val);
        self.vertices.insert(vertex.clone());
        self.vertices.get(&vertex).unwrap()
    }

    pub fn contains_vertex(&self, val: &V) -> bool {
        self.vertices.contains(val)
    }
}

// ******************************************************************
// **********          Unweighted                          **********
// ******************************************************************
impl<V: Node, D: EdgeDirection> Graph<V, D, UnweightedEdge> {
    // returns edge reference or None if nothing was added (invalid input)
    // inputs must have `self` lifetime, so they're valid unless deleted
    // can't just take V as input: would need to create Vertex<&V> :/
    pub fn add_edge<'a>(&'a mut self, l: &'a Vertex<V>, r: &'a Vertex<V>) 
        -> Option<&'a Edge<V,D,UnweightedEdge>> {
        let lhs = self.vertices.get(l)?;
        let rhs = self.vertices.get(r)?;
        let edge = Edge::between(lhs.clone(), rhs.clone());
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
// **********          Directed, Weighted                  **********
// ******************************************************************
impl<V: Node> Graph<V, DirectedEdge, WeightedEdge> {
    pub fn new_directed_weighted() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Undirected, Weighted                **********
// ******************************************************************
impl<V: Node> Graph<V, UndirectedEdge, WeightedEdge> {
    pub fn new_undirected_weighted() -> Self {
        Self::new()
    }
}



