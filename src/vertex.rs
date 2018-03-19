// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use edge::{Edge,};
use edge::{EdgeWeight};
use edge::{EdgeDir, DirectedEdge, UndirectedEdge};

use addr_hm::{VertAddr, EdgeAddr};

use std::fmt;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::marker::PhantomData;

pub trait NodeT: fmt::Debug + Eq + Hash {}
impl<T: fmt::Debug + Eq + Hash> NodeT for T {}

pub trait VertexDir: fmt::Debug + Default {
    type EdgePair: EdgeDir<VertexPair=Self>;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>>;
    fn register_parent(&mut self, edge: EdgeAddr);
    fn register_child(&mut self, edge: EdgeAddr);
}
#[derive(Debug)] 
pub struct DirectedVertex {
    parents: Vec<EdgeAddr>,
    children: Vec<EdgeAddr>,
    //_x: PhantomData<W>,
}
#[derive(Debug)] 
pub struct UndirectedVertex {
    neighbors: Vec<EdgeAddr>,
}
impl VertexDir for DirectedVertex {
    type EdgePair = DirectedEdge;
    //fn get_neighbors<'a>(&'a self) -> Box<Iterator<Item=&'a Vertex<V>>> {
        //self.parents.iter().map(Edge::get_src)
    //}
    fn register_parent(&mut self, edge: EdgeAddr) {
        self.parents.push(edge);
    }
    fn register_child(&mut self, edge: EdgeAddr) {
        self.children.push(edge);
    }
}
impl VertexDir for UndirectedVertex {
    type EdgePair = UndirectedEdge;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>> {
        //unimplemented!()
    //}
    fn register_parent(&mut self, edge: EdgeAddr) {
        self.neighbors.push(edge);
    }
    fn register_child(&mut self, edge: EdgeAddr) {
        self.neighbors.push(edge);
    }
}

impl Default for DirectedVertex {
    fn default() -> Self {
        DirectedVertex {
            parents: vec![],
            children: vec![],
        }
    }
}
impl Default for UndirectedVertex {
    fn default() -> Self {
        UndirectedVertex {
            neighbors: vec![],
        }
    }
}

#[derive(Debug)] 
pub struct Vertex<V: NodeT, D: EdgeDir> {
    val: V,
    dir: D::VertexPair,
    //_w: PhantomData<W>,
}

impl<V: NodeT, D: EdgeDir> Vertex<V,D> {
    pub(crate) fn new(val: V) -> Self { 
        Vertex {
            val,
            dir: D::VertexPair::default(),
        }
    }
    pub(crate) fn register_child_edge(&mut self, edge: EdgeAddr) {
        self.dir.register_child(edge);
    }
    pub(crate) fn register_parent_edge(&mut self, edge: EdgeAddr) {
        self.dir.register_parent(edge);
    }
}


/*
impl<V: NodeT, D, W: EdgeWeight, P> Vertex<V,D,W> 
    where D: EdgeDir<W, VertexPair=P>,
          P: VertexDir<W, EdgePair=D> 
{ }
*/


impl<V: NodeT> Vertex<V, DirectedEdge> {
    pub(crate) fn register_parent(&mut self, edge: EdgeAddr) {
        self.dir.parents.push(edge);
    }
    pub(crate) fn register_child(&mut self, edge: EdgeAddr) {
        self.dir.children.push(edge);
    }
}

impl<V: NodeT> Vertex<V, UndirectedEdge> {
    pub(crate) fn register_neighbor(&mut self, edge: EdgeAddr) {
        self.dir.neighbors.push(edge);
    }
}

impl<V: NodeT, D: EdgeDir> Borrow<V> for Vertex<V,D> {
    fn borrow(&self) -> &V {
        &self.val
    }

}


// Define equivalence to be based only on the generic `val`
// Could this ever cause data corruption? I don't think so
// Users can't add a second identical elem (it'll just replace the first)

impl<V: NodeT, D: EdgeDir> PartialEq for Vertex<V,D> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl<V: NodeT, D: EdgeDir> Eq for Vertex<V,D> { }
impl<V: NodeT, D: EdgeDir> Hash for Vertex<V,D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
//impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> NodeT for Vertex<V,D,W> { }


