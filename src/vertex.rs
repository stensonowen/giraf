// TODO
//  make sure Vertex::Hash just shims T::hash (so users can define their own equality)

use edge::{Edge,};
use edge::{EdgeWeight};
use edge::{EdgeDir, DirectedEdge, UndirectedEdge};

use addr_hm::Addr;

use std::fmt;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use std::marker::PhantomData;

pub trait NodeT: fmt::Debug + Eq + Hash {}

pub trait VertexDir<V: NodeT, W: EdgeWeight>: fmt::Debug + Default {
    type EdgePair: EdgeDir<W, VertexPair=Self>;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>>;
    //fn register_parent(&mut self, edge: Edge<Self::EdgePair,W>);
    //fn register_child(&mut self, edge: Edge<Self::EdgePair,W>);
}
#[derive(Debug)] 
pub struct DirectedVertex<V: NodeT, W: EdgeWeight> {
    //parents: Vec<Edge<DirectedEdge, W>>,
    //children: Vec<Edge<DirectedEdge, W>>,
    parents: Vec<Addr<Edge<V, DirectedEdge, W>>>,
    children: Vec<Addr<Edge<V, DirectedEdge, W>>>,
    _x: PhantomData<W>,
}
#[derive(Debug)] 
pub struct UndirectedVertex<V: NodeT, W: EdgeWeight> {
    //neighbors: Vec<Edge<UndirectedEdge, W>>,
    neighbors: Vec<Addr<Edge<V, UndirectedEdge, W>>>,
    _x: PhantomData<W>,
}
impl<V: NodeT, W: EdgeWeight> VertexDir<V, W> for DirectedVertex<V, W> {
    type EdgePair = DirectedEdge;
    //fn get_neighbors<'a>(&'a self) -> Box<Iterator<Item=&'a Vertex<V>>> {
        //self.parents.iter().map(Edge::get_src)
    //}
    /*
    fn register_parent(&mut self, edge: Edge<Self::EdgePair,W>) {
        self.parents.push(edge);
    }
    fn register_child(&mut self, edge: Edge<Self::EdgePair,W>) {
        self.children.push(edge);
    }
    */
}
impl<V: NodeT, W: EdgeWeight> VertexDir<V, W> for UndirectedVertex<V, W> {
    type EdgePair = UndirectedEdge;
    //fn get_neighbors(&self) -> Box<Iterator<Item=&Vertex<V>>> {
        //unimplemented!()
    //}
    /*
    fn register_parent(&mut self, edge: Edge<Self::EdgePair,W>) {
        self.neighbors.push(edge);
    }
    fn register_child(&mut self, edge: Edge<Self::EdgePair,W>) {
        self.neighbors.push(edge);
    }
    */
}

impl<V: NodeT, W: EdgeWeight> Default for DirectedVertex<V, W> {
    fn default() -> Self {
        DirectedVertex {
            parents: vec![],
            children: vec![],
            _x: PhantomData,
        }
    }
}
impl<V: NodeT, W: EdgeWeight> Default for UndirectedVertex<V, W> {
    fn default() -> Self {
        UndirectedVertex {
            neighbors: vec![],
            _x: PhantomData,
        }
    }
}

#[derive(Debug)] 
pub struct Vertex<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> {
    val: V,
    dir: <D as EdgeDir<W>>::VertexPair,
    _w: PhantomData<W>,
}

/*
impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> Vertex<V,D,W> {
    pub(crate) fn new(val: V) -> Self { 
        Vertex {
            val,
            dir: <D as EdgeDir<W>>::VertexPair::default(),
            _w: PhantomData,
        }
    }
    pub(crate) fn register_child_edge(&mut self, edge: Edge<D,W>) {
        self.dir.register_child(edge);
    }
    pub(crate) fn register_parent_edge(&mut self, edge: Edge<D,W>) {
        self.dir.register_parent(edge);
    }
}


/*
impl<V: NodeT, D, W: EdgeWeight, P> Vertex<V,D,W> 
    where D: EdgeDir<W, VertexPair=P>,
          P: VertexDir<W, EdgePair=D> 
{ }
*/


impl<V: NodeT, W: EdgeWeight> Vertex<V, DirectedEdge, W> {
    /*
    fn register_parent(&mut self, edge: Edge<DirectedEdge, W>) {
        self.dir.parents.push(edge);
    }
    fn register_child(&mut self, edge: Edge<DirectedEdge, W>) {
        self.dir.children.push(edge);
    }
    */
}

impl<V: NodeT, W: EdgeWeight> Vertex<V, UndirectedEdge, W> {
    /*
    fn register_neighbor(&mut self, edge: Edge<UndirectedEdge, W>) {
        self.dir.neighbors.push(edge);
    }
    */
}
*/

impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> Borrow<V> for Vertex<V,D,W> {
    fn borrow(&self) -> &V {
        &self.val
    }

}


// Define equivalence to be based only on the generic `val`
// Could this ever cause data corruption? I don't think so
// Users can't add a second identical elem (it'll just replace the first)

impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> PartialEq for Vertex<V,D,W> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> Eq for Vertex<V,D,W> { }
impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> Hash for Vertex<V,D,W> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
impl<V: NodeT, D: EdgeDir<W>, W: EdgeWeight> NodeT for Vertex<V,D,W> { }

