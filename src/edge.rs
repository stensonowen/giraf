// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use vertex::{Vertex, Node};
use vertex::{VertexDir, DirectedVertex, UndirectedVertex};
use addr_hm::Addr;


// ********************************************************
// **********          Edge Directions           **********
// ********************************************************

pub trait EdgeDir<W: EdgeWeight>: fmt::Debug + Default {
    type VertexPair: VertexDir<W, EdgePair=Self>;
}
#[derive(Debug, Default)] pub struct DirectedEdge;
#[derive(Debug, Default)] pub struct UndirectedEdge;
impl<W: EdgeWeight> EdgeDir<W> for DirectedEdge {
    type VertexPair = DirectedVertex<W>;
}
impl<W: EdgeWeight> EdgeDir<W> for UndirectedEdge {
    type VertexPair = UndirectedVertex<W>;
}


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

#[derive(Debug, Clone)]
pub struct Edge<D: EdgeDir<W>, W: EdgeWeight> {
    // if Directed, edge goes from left to right
    dir: D,
    weight: W,
    lhs: Addr,
    rhs: Addr,
}

// ********************************************************
// **********          Edge                      **********
// ********************************************************
impl<D: EdgeDir<W>, W: EdgeWeight> Edge<D, W> {
    pub(crate) fn between(l: Addr, r: Addr, w: W::Weight) -> Self {
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
impl<D: EdgeDir<UnweightedEdge>> Edge<D, UnweightedEdge> {
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
impl<D: EdgeDir<UnsignedEdge>> Edge<D, UnsignedEdge> {
    //pub(crate) fn between_(lhs: Vertex<N>, rhs: Vertex<N>) -> Self { }
}

// ********************************************************
// **********          Undirected Edge           **********
// ********************************************************
impl<W: EdgeWeight> Edge<UndirectedEdge, W> {
    pub(crate) fn get_ends(&self) -> [&Addr;2] {
        [&self.lhs, &self.rhs]
    }

}

// ********************************************************
// **********          Directed Edge             **********
// ********************************************************
impl<W: EdgeWeight> Edge<DirectedEdge, W> {
    pub(crate) fn get_src(&self) -> &Addr {
        &self.lhs
    }
    pub(crate) fn get_dst(&self) -> &Addr {
        &self.rhs
    }
}

