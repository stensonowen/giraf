// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use std::hash::Hash;

use vertex::{Vertex, NodeT};
use vertex::{VertexDir, DirectedVertex, UndirectedVertex};
use addr_collec::VertAddr;


pub trait EdgeWeight: fmt::Debug + Hash + Eq {}
#[derive(Debug, Hash, PartialEq, Eq)] pub struct UnweightedEdge;
impl EdgeWeight for UnweightedEdge {}
//impl<T: fmt::Debug> EdgeT for T {}

// ********************************************************
// **********          Edge Directions           **********
// ********************************************************

pub trait EdgeDir: fmt::Debug + Default + Eq + Hash {
    type VertexPair: VertexDir<EdgePair=Self>;
}
#[derive(Debug, Default, PartialEq, Eq, Hash)] pub struct DirectedEdge;
#[derive(Debug, Default, PartialEq, Eq, Hash)] pub struct UndirectedEdge;
impl EdgeDir for DirectedEdge {
    type VertexPair = DirectedVertex;
}
impl EdgeDir for UndirectedEdge {
    type VertexPair = UndirectedVertex;
}


// ********************************************************
// **********          Edge Weights              **********
// ********************************************************

/*
pub trait EdgeWeight: fmt::Debug + Hash + Eq {
    type Weight;
    fn new(w: Self::Weight) -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq)] pub struct UnweightedEdge;
#[derive(Debug, Hash, PartialEq, Eq)] pub struct UnsignedEdge(u32);
#[derive(Debug, Hash, PartialEq, Eq)] pub struct SignedEdge(i32);

impl EdgeWeight for UnweightedEdge {
    type Weight = ();
    fn new(w: Self::Weight) -> Self { UnweightedEdge }
}
impl EdgeWeight for UnsignedEdge {
    type Weight = u32;
    fn new(w: Self::Weight) -> Self { UnsignedEdge(w) }
}
impl EdgeWeight for SignedEdge {
    type Weight = i32;
    fn new(w: Self::Weight) -> Self { SignedEdge(w) }
}
*/



// ********************************************************
// **********          Edge                      **********
// ********************************************************

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge<D: EdgeDir, W: EdgeWeight> {
    // if Directed, edge goes from left to right
    dir: D,
    weight: W,
    lhs: VertAddr,
    rhs: VertAddr,
}


// ********************************************************
// **********          Edge                      **********
// ********************************************************
impl<D: EdgeDir, W: EdgeWeight> Edge<D, W> {
    pub(crate) fn between(l: VertAddr, r: VertAddr, w: W) -> Self {
        Edge {
            dir: D::default(),
            //weight: W::new(w),
            weight: w,
            lhs: l,
            rhs: r,
        }
    }
}

// ********************************************************
// **********          Unweighted Edge           **********
// ********************************************************
impl<D: EdgeDir> Edge<D, UnweightedEdge> {
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

/*
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

*/
