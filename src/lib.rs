#![allow(unused)]

use std::fmt;
use std::collections::HashMap;

mod edge;
use edge::{Edge};
use edge::{EdgeDirection, DirectedEdge, UndirectedEdge};
use edge::{EdgeWeight, WeightedEdge, UnweightedEdge};

mod vertex;
use vertex::{Vertex, Node};

type AddrType = usize;
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Addr(AddrType);

/*
pub trait Node: fmt::Debug {}

#[derive(Debug)] 
struct Vertex<T: Node> {
    val: T,
    addr: Addr, // is this necessary?
}
*/

#[derive(Debug)]
pub struct Graph<V: Node, D: EdgeDirection, W: EdgeWeight> {
    // `State` element: can store counter if in building state
    // maybe use `rental` to refer to self? does that mean we can't move G?
    addresses: HashMap<Addr, Vertex<V>>,
    edges: Vec<Edge<D,W>>,
    counter: AddrType,
}

impl<V: Node, D: EdgeDirection, W: EdgeWeight> Graph<V,D,W> {
    pub fn new() -> Self {
        Graph {
            addresses: HashMap::new(),
            edges: Vec::new(),
            counter: 0
        }
    }
    /// A vertex can be added to any graph, regarrdless of edge weighted-ness or directed-ness
    pub fn add_vertex(&mut self, val: V) {
        let addr = Addr(self.counter);
        self.counter += 1;
        let vertex = Vertex::from(val, addr);
        self.addresses.insert(addr, vertex);
    }

}

// ******************************************************************
// **********          Unweighted                          **********
// ******************************************************************
impl<V: Node, D: EdgeDirection> Graph<V, D, UnweightedEdge> {
    pub fn add_edge(&mut self, lhs: &V, rhs: &V) {
        // if lhs or rhs are absent...
        //  panic?
        //  add them?
        //  return None or something?
        //let left = self.
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



