// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use std::rc::Rc;
use vertex::{Vertex, Node};


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

pub trait EdgeWeight: fmt::Debug {
    type Weight;
    fn new(w: Self::Weight) -> Self;
}

#[derive(Debug)] pub struct UnweightedEdge;
#[derive(Debug)] pub struct UnsignedEdge(u32);
#[derive(Debug)] pub struct SignedEdge(i32);

impl EdgeWeight for UnweightedEdge {
    type Weight = ();
    fn new(_: ()) -> Self { UnweightedEdge }
}
impl EdgeWeight for UnsignedEdge {
    type Weight = u32;
    fn new(u: u32) -> Self { UnsignedEdge(u) }
}
impl EdgeWeight for SignedEdge {
    type Weight = i32;
    fn new(i: i32) -> Self { SignedEdge(i) }
}



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
// **********          Edge                      **********
// ********************************************************
impl<N: Node, D: EdgeDirection, W: EdgeWeight> Edge<N, D, W> {
    pub fn between(l: Vertex<N>, r: Vertex<N>, w: W::Weight) -> Self {
        Edge {
            dir: D::default(),
            weight: W::new(w),
            lhs: l,
            rhs: r,
        }
    }
}

// ********************************************************
// **********          Unweighted Edge           **********
// ********************************************************
impl<N: Node, D: EdgeDirection> Edge<N, D, UnweightedEdge> {
    /*
    pub(crate) fn between(lhs: Vertex<N>, rhs: Vertex<N>) -> Self {
        Edge {
            dir: D::default(),
            weight: UnweightedEdge,
            lhs, rhs
        }
    }
    */
}

// ********************************************************
// **********          Edge with Unsigned Weights**********
// ********************************************************
impl<N: Node, D: EdgeDirection> Edge<N, D, UnsignedEdge> {
    //pub(crate) fn between_(lhs: Vertex<N>, rhs: Vertex<N>) -> Self { }
}

// ********************************************************
// **********          Undirected Edge           **********
// ********************************************************
impl<N: Node, W: EdgeWeight> Edge<N, UndirectedEdge, W> {
    //pub(crate) fn between(lhs: Vertex<N>, rhs: Vertex<N>, 

}
