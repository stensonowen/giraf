// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use std::rc::Rc;
use vertex::{Vertex, Node};

//use Addr;


// ********************************************************
// **********          Edge Directions           **********
// ********************************************************

pub trait EdgeDirection: fmt::Debug + Default {}
#[derive(Debug, Default)] pub struct DirectedEdge;
#[derive(Debug, Default)] pub struct UndirectedEdge;
impl EdgeDirection for DirectedEdge {}
impl EdgeDirection for UndirectedEdge {}


// ********************************************************
// **********          Edge Weights              **********
// ********************************************************

pub trait EdgeWeight: fmt::Debug { }
#[derive(Debug)] pub struct UnweightedEdge;
#[derive(Debug)] pub struct UnsignedEdge(u32);
#[derive(Debug)] pub struct SignedEdge(i32);
impl EdgeWeight for UnweightedEdge { }
impl EdgeWeight for UnsignedEdge { }
impl EdgeWeight for SignedEdge { }



// ********************************************************
// **********          Edge                      **********
// ********************************************************

#[derive(Debug)]
pub struct Edge<N: Node, D: EdgeDirection, W: EdgeWeight> {
    // if Directed, edge goes from left to right
    dir: D,
    weight: W,
    lhs: Vertex<N>,
    rhs: Vertex<N>,
}

// ********************************************************
// **********          Unweighted Edge           **********
// ********************************************************
impl<N: Node, D: EdgeDirection> Edge<N, D, UnweightedEdge> {
    pub(crate) fn between(lhs: Vertex<N>, rhs: Vertex<N>) -> Self {
        Edge {
            dir: D::default(),
            weight: UnweightedEdge,
            lhs, rhs
        }
    }
}

// ********************************************************
// **********          Undirected Edge           **********
// ********************************************************
impl<N: Node, W: EdgeWeight> Edge<N, UndirectedEdge, W> {
    //pub(crate) fn between(lhs: Vertex<N>, rhs: Vertex<N>, 

}
