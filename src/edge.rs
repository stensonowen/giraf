// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use vertex::{Vertex, Node};
use vertex::{VertexDirection, DirectedVertex, UndirectedVertex};


// ********************************************************
// **********          Edge Directions           **********
// ********************************************************

pub trait EdgeDir<V: Node, W: EdgeWeight>: fmt::Debug + Default + Eq + Hash {
    type VertexPair: VertexDirection<V, W>;
}
#[derive(Debug, Default, PartialEq, Eq, Hash)] pub struct DirectedEdge;
#[derive(Debug, Default, PartialEq, Eq, Hash)] pub struct UndirectedEdge;
impl<V: Node, W: EdgeWeight> EdgeDir<V, W> for DirectedEdge {
    type VertexPair = DirectedVertex<V,W>;
}
impl<V: Node, W: EdgeWeight> EdgeDir<V, W> for UndirectedEdge {
    type VertexPair = UndirectedVertex<V,W>;
}


// ********************************************************
// **********          Edge Weights              **********
// ********************************************************

pub trait EdgeWeight: fmt::Debug + Eq + Hash {
    type Weight;
    fn new(w: Self::Weight) -> Self;
}

#[derive(Debug, PartialEq, Eq, Hash)] pub struct UnweightedEdge;
#[derive(Debug, PartialEq, Eq, Hash)] pub struct UnsignedEdge(u32);
#[derive(Debug, PartialEq, Eq, Hash)] pub struct SignedEdge(i32);

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
pub struct Edge<N: Node, D: EdgeDir<N,W>, W: EdgeWeight> {
    // if Directed, edge goes from left to right
    dir: D,
    weight: W,
    lhs: Vertex<N,D,W>,
    rhs: Vertex<N,D,W>,
}

// ********************************************************
// **********          Edge                      **********
// ********************************************************
impl<N: Node, D: EdgeDir<N,W>, W: EdgeWeight> Edge<N, D, W> {
    pub fn between(l: Vertex<N,D,W>, r: Vertex<N,D,W>, w: W::Weight) -> Self {
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
impl<N: Node, D: EdgeDir<N,UnweightedEdge>> Edge<N, D, UnweightedEdge> {
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
impl<N: Node, D: EdgeDir<N,UnsignedEdge>> Edge<N, D, UnsignedEdge> {
    //pub(crate) fn between_(lhs: Vertex<N>, rhs: Vertex<N>) -> Self { }
}

// ********************************************************
// **********          Undirected Edge           **********
// ********************************************************
impl<N: Node, W: EdgeWeight> Edge<N, UndirectedEdge, W> {
    pub(crate) fn get_ends(&self) -> [&Vertex<N,UndirectedEdge,W>;2] {
        [&self.lhs, &self.rhs]
    }

}

// ********************************************************
// **********          Directed Edge             **********
// ********************************************************
impl<N: Node, W: EdgeWeight> Edge<N, DirectedEdge, W> {
    pub(crate) fn get_src(&self) -> &Vertex<N, DirectedEdge, W> {
        &self.lhs
    }
    pub(crate) fn get_dst(&self) -> &Vertex<N, DirectedEdge, W> {
        &self.rhs
    }
}
