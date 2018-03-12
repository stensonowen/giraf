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

mod addr_hm;
use addr_hm::{Addr, AddrSet};

#[derive(Debug)]
pub struct Graph<V: Node, D: EdgeDir<V,W>, W: EdgeWeight> {
    // `State` element: can store counter if in building state
    // maybe use `rental` to refer to self? does that mean we can't move G?

    //edges: Vec<Edge<V,D,W>>,
    //vertices: HashSet<Vertex<V,D,W>>,

    //nodes: HashMap<Vertex<V>, Vec<Edge<V,D,W>>>,
    //nodes: HashSet<Vertex<V,D,W>>,
    nodes: AddrSet<Vertex<V,D,W>>,

}

impl<V: Node, D: EdgeDir<V,W>, W: EdgeWeight> Graph<V,D,W> {
    pub fn new() -> Self {
        Graph {
            //edges: Vec::new(),
            //vertices: HashSet::new(),
            //nodes: HashSet::new(),
            nodes: AddrSet::default()
        }
    }
    /*
    /// A vertex can be added to any graph, 
    /// regardless of edge weighted-ness or directed-ness
    pub fn add_vertex(&mut self, val: V) -> &Vertex<V,D,W> {
        let vertex = Vertex::new(val);
        self.nodes.insert(vertex.clone());
        self.nodes.get(&vertex).unwrap()
    }
    */

    pub fn contains_value(&self, val: &V) -> bool {
        self.nodes.contains(val)
    }
    pub fn contains_vertex(&self, val: &Vertex<V,D,W>) -> bool {
        self.nodes.contains(val)
    }
}

// ******************************************************************
// **********          Unweighted                          **********
// ******************************************************************
impl<V: Node, D: EdgeDir<V,UnweightedEdge>> Graph<V, D, UnweightedEdge> {
    // returns edge reference or None if nothing was added (invalid input)
    // inputs must have `self` lifetime, so they're valid unless deleted
    // can't just take V as input: would need to create Vertex<&V> :/
    /*
    pub fn add_edge<'a>(&'a mut self, 
                        l: &'a Vertex<V, D, UnweightedEdge>, 
                        r: &'a Vertex<V, D, UnweightedEdge>) 
                        -> Option<&'a Edge<V,D,UnweightedEdge>> {
        let lhs = self.nodes.get(l)?;
        let rhs = self.nodes.get(r)?;
        let edge = Edge::between(lhs.clone(), rhs.clone(), ());
        self.edges.push(edge);
        self.edges.last() // uhh this part should never be none
    }
    */
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
    // returns edge reference or None if nothing was added (invalid input)
    // inputs must have `self` lifetime, so they're valid unless deleted
    // could just take &V as input... would that be better?
    /*
    pub fn add_edge<'a>(&'a mut self, 
                        l: &'a Vertex<V, DirectedEdge, UnweightedEdge>, 
                        r: &'a Vertex<V, DirectedEdge, UnweightedEdge>) 
                        -> Option<&'a Edge<V, DirectedEdge, UnweightedEdge>>
    {
        let lhs = self.nodes.get(l)?;
        let rhs = self.nodes.get(r)?;
        let l_edge = Edge::between(lhs.clone(), rhs.clone(), ());
        let r_edge = Edge::between(lhs.clone(), rhs.clone(), ());
        //self.edges.push(edge);
        None
    }
    */
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
