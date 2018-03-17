// TODO
//  make edge weights generic? anything that can be added/compared or something?

use std::fmt;
use std::hash::Hash;

use vertex::{Vertex, NodeT};
use vertex::{VertexDir, DirectedVertex, UndirectedVertex};
use addr_hm::{VertAddr};
//use addr_hm::Addr;


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

pub trait EdgeWeight: fmt::Debug + Hash + Eq { }

#[derive(Debug, Hash, PartialEq, Eq)] pub struct UnweightedEdge;
#[derive(Debug, Hash, PartialEq, Eq)] pub struct UnsignedEdge(u32);
#[derive(Debug, Hash, PartialEq, Eq)] pub struct SignedEdge(i32);

impl EdgeWeight for UnweightedEdge { }
impl EdgeWeight for UnsignedEdge { }
impl EdgeWeight for SignedEdge { }



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


/*
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

*/
