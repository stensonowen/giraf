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

pub trait EdgeWeight: fmt::Debug {
    fn get_weight(&self) -> usize;
}
#[derive(Debug)] pub struct UnweightedEdge;
#[derive(Debug)] pub struct WeightedEdge(usize);
impl EdgeWeight for UnweightedEdge {
    fn get_weight(&self) -> usize { 0 }
}
impl EdgeWeight for WeightedEdge {
    fn get_weight(&self) -> usize { self.0 }
}



// ********************************************************
// **********          Edge                      **********
// ********************************************************

#[derive(Debug)]
pub struct Edge<N: Node, D: EdgeDirection, W: EdgeWeight> {
    // if Directed, edge goes from left to right
    dir: D,
    weight: W,
    //left: Addr,
    //right: Addr,
    lhs: Rc<Vertex<N>>,
    rhs: Rc<Vertex<N>>,
}

// ********************************************************
// **********          Unweighted Edge           **********
// ********************************************************
impl<N: Node, D: EdgeDirection> Edge<N, D, UnweightedEdge> {
    pub(crate) fn between(lhs: Rc<Vertex<N>>, rhs: Rc<Vertex<N>>) -> Self {
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
//impl<W: 
