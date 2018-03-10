// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use edge::{Edge,};
use edge::{EdgeWeight};
use edge::{EdgeDirection, DirectedEdge, UndirectedEdge};

use std::fmt;
use std::rc::Rc;
use std::hash::Hash;
use std::borrow::Borrow;
use std::marker::PhantomData;

pub trait Node: fmt::Debug + Eq + Hash {}

pub trait VertexDirection<V: Node, W: EdgeWeight>: fmt::Debug {
    type EdgePair: EdgeDirection<V, W>;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>>;
}
#[derive(Debug)] 
pub struct DirectedVertex<V: Node, W: EdgeWeight> {
    parents: Vec<Edge<V, DirectedEdge, W>>,
    children: Vec<Edge<V, DirectedEdge, W>>,
}
#[derive(Debug)] 
pub struct UndirectedVertex<V: Node, W: EdgeWeight> {
    neighbors: Vec<Edge<V, UndirectedEdge, W>>,
}
impl<V: Node, W: EdgeWeight> VertexDirection<V, W> for DirectedVertex<V, W> {
    type EdgePair = DirectedEdge;
    //fn get_neighbors<'a>(&'a self) -> Box<Iterator<Item=&'a Vertex<V>>> {
        //self.parents.iter().map(Edge::get_src)
    //}
}
impl<V: Node, W: EdgeWeight> VertexDirection<V, W> for UndirectedVertex<V, W> {
    type EdgePair = UndirectedEdge;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>> {
        //unimplemented!()
    //}
}

#[derive(Debug, Hash, PartialEq, Eq)] 
pub struct VertexInner<V: Node, D: EdgeDirection<V,W>, W: EdgeWeight> {
    val: V,
    dir: D,
    _w: PhantomData<W>,
}


#[derive(Debug, Hash, PartialEq, Eq)] 
pub struct Vertex<T: Node>(Rc<T>); 

impl<T: Node> Vertex<T> {
    pub(crate) fn new(val: T) -> Self { 
        Vertex(Rc::new(val))
        //Vertex { val: Rc::new(val), }
    }
}

impl<T: Node> Borrow<T> for Vertex<T> {
    fn borrow(&self) -> &T {
        //&self.val
        &self.0
    }

}

impl<T: Node> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex(self.0.clone())
        //Vertex { val: self.val.clone() }
    }
}

