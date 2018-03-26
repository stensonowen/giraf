#![allow(unused)]

/*
 * proper way to store edges? given that they're probably non-unique?
 *  maybe just only insert into AddrHS if absent?
 *  maybe use a vector and just push everything?
 *      never need to look up an edge by its weight or anything
 */

use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

mod edge;
use edge::{Edge};
use edge::{EdgeDir, DirectedEdge, UndirectedEdge};
//use edge::{EdgeWeight, SignedEdge, UnsignedEdge, UnweightedEdge};
use edge::{EdgeWeight, UnweightedEdge};

mod vertex;
use vertex::{Vertex, NodeT};

mod addr_collec;
use addr_collec::{AddrSet, VertAddr, EdgeAddr};

#[derive(Debug)]
pub struct Graph<V: NodeT, D: EdgeDir, W: EdgeWeight> {
    // consider `State` element: can store counter if in building state
    nodes: AddrSet<Vertex<V,D>, VertAddr>,
    edges: AddrSet<Edge<D,W>, EdgeAddr>,

}

impl<V: NodeT, D: EdgeDir, W: EdgeWeight> Graph<V,D,W> {
    pub fn new() -> Self {
        Graph {
            nodes: AddrSet::default(),
            edges: AddrSet::default(),
        }
    }
    /// A vertex can be added to any graph, 
    /// regardless of edge weighted-ness or directed-ness
    pub fn add_vertex(&mut self, val: V) -> &Vertex<V,D> {
        let vertex = Vertex::new(val);
        let addr = self.nodes.insert(vertex).unwrap();
        &self.nodes[addr]
    }

    pub fn contains_value(&self, val: &V) -> bool {
        self.nodes.contains(val)
    }
    pub fn contains_vertex(&self, val: &Vertex<V,D>) -> bool {
        self.nodes.contains(val)
    }
}

// ******************************************************************
// **********          Unweighted                          **********
// ******************************************************************
impl<V: NodeT, D: EdgeDir> Graph<V, D, UnweightedEdge> {
    // returns edge reference or None if nothing was added (invalid input)
    // inputs must have `self` lifetime, so they're valid unless deleted
    // can't just take V as input: would need to create Vertex<&V> :/
    /*
    pub fn add_edge<'a>(&'a mut self, l: &'a Vertex<V,D>, r: &'a Vertex<V,D>)
        -> Option<EdgeAddr> 
    {
        let lhs = self.nodes.get(l)?;
        let rhs = self.nodes.get(r)?;
        let edge = Edge::between(lhs, rhs, ());
        let edge_addr = self.edges.insert(edge)?;
        self.nodes[lhs].register_child_edge(edge_addr);
        self.nodes[rhs].register_parent_edge(edge_addr);
        Some(edge_addr)
    }
    */
}


// ******************************************************************
// **********          Undirected                          **********
// ******************************************************************
impl<V: NodeT, W: EdgeWeight> Graph<V, UndirectedEdge, W> {

}

// ******************************************************************
// **********          Undirected, Unweighted              **********
// ******************************************************************

impl<V: NodeT> Graph<V, UndirectedEdge, UnweightedEdge> {
    pub fn new_undirected_unweighted() -> Self {
        Self::new()
    }
    pub fn add_edge<'a>(&'a mut self, 
                        l: &'a Vertex<V, UndirectedEdge>, 
                        r: &'a Vertex<V, UndirectedEdge>) 
        -> Option<EdgeAddr> 
    {
        let lhs = self.nodes.get(l)?;
        let rhs = self.nodes.get(r)?;
        let edge = Edge::between(lhs.clone(), rhs.clone(), UnweightedEdge);
        let edge_addr = self.edges.insert(edge)?;
        self.nodes[lhs].register_neighbor(edge_addr.clone());
        self.nodes[rhs].register_neighbor(edge_addr.clone());
        Some(edge_addr)
    }
}

// ******************************************************************
// **********          Directed, Unweighted                **********
// ******************************************************************
impl<V: NodeT> Graph<V, DirectedEdge, UnweightedEdge> {
    pub fn new_directed_unweighted() -> Self {
        Self::new()
    }
    pub fn add_edge<'a>(&'a mut self, 
                        l: &'a Vertex<V, DirectedEdge>, 
                        r: &'a Vertex<V, DirectedEdge>) 
        -> Option<EdgeAddr> 
    {
        let lhs = self.nodes.get(l)?;
        let rhs = self.nodes.get(r)?;
        let edge = Edge::between(lhs.clone(), rhs.clone(), UnweightedEdge);
        let edge_addr = self.edges.insert(edge)?;
        self.nodes[lhs].register_child(edge_addr.clone());
        self.nodes[rhs].register_parent(edge_addr.clone());
        Some(edge_addr)
    }
}

/*
// ******************************************************************
// **********          Directed, Unsigned Weights          **********
// ******************************************************************
impl<V: NodeT> Graph<V, DirectedEdge, UnsignedEdge> {
    pub fn new_directed_weighted() -> Self {
        Self::new()
    }
}
// ******************************************************************
// **********          Directed, Signed Weights            **********
// ******************************************************************
impl<V: NodeT> Graph<V, DirectedEdge, SignedEdge> {
    pub fn new_directed_weighted_signed() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Undirected, Unsigned Weights        **********
// ******************************************************************
impl<V: NodeT> Graph<V, UndirectedEdge, UnsignedEdge> {
    pub fn new_undirected_weighted() -> Self {
        Self::new()
    }
}

// ******************************************************************
// **********          Undirected, Signed Weights          **********
// ******************************************************************
impl<V: NodeT> Graph<V, UndirectedEdge, SignedEdge> {
    pub fn new_undirected_weighted_signed() -> Self {
        Self::new()
    }
}
*/


// hmmmm
//  can I use one vec of edges and only store slices of it?
//  every edge will be incident to 2 vertices (may be the same)
//  dude i don't fucken know it's too late
// todo
//  unify letters used for Vertex data (T/V/N)

