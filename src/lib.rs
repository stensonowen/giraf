
use std::fmt;
use std::mem;
use std::hash::Hash;
use std::marker::PhantomData;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
//  Direction
///////////////////////////////////////////////////////////////////////////////

pub trait DirT<E: EdgeT>: fmt::Debug {
    fn new() -> Self;
}

#[derive(Debug)] 
struct Dir<V: NodeT, E: EdgeT> {
    children: Vec<*const Edge<V, E, Dir<V, E>>>,
    parents: Vec<*const Edge<V, E, Dir<V, E>>>,
}

#[derive(Debug)] 
struct Undir<V: NodeT, E: EdgeT> {
    neighbors: Vec<*const Edge<V, E, Undir<V,E>>>,
}

impl<V: NodeT, E: EdgeT> DirT<E> for Dir<V,E> {
    fn new() -> Self { Dir { children: vec![], parents: vec![] } }
}
impl<V: NodeT, E: EdgeT> DirT<E> for Undir<V,E> {
    fn new() -> Self { Undir { neighbors: vec![] } }
}

///////////////////////////////////////////////////////////////////////////////
//  Vertex
///////////////////////////////////////////////////////////////////////////////

pub trait NodeT: 'static + fmt::Debug + Eq + Hash {}
impl<T: 'static + fmt::Debug + Eq + Hash> NodeT for T {}

#[derive(Debug)]
struct Vertex<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: Box<V>,
    hood: D,
    _e: ::std::marker::PhantomData<E>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Vertex<V,E,D> {
    fn new(val: V) -> Self {
        Vertex {
            val: Box::new(val),
            hood: D::new(),
            _e: PhantomData,
        }
    }
    fn borrow(&self) -> &V {
        &self.val
    }
}

///////////////////////////////////////////////////////////////////////////////
//  Edge
///////////////////////////////////////////////////////////////////////////////

pub trait EdgeT: fmt::Debug {}
impl<T: fmt::Debug> EdgeT for T {}

#[derive(Debug)]
struct Edge<V: NodeT, E: EdgeT, D: DirT<E>> {
    val: E,
    lhs: *const Vertex<V,E,D>,
    rhs: *const Vertex<V,E,D>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Edge<V,E,D> {
    fn new(e: E, l: &Vertex<V,E,D>, r: &Vertex<V,E,D>) -> Self {
        Edge {
            val: e,
            lhs: l as *const Vertex<V,E,D>,
            rhs: r as *const Vertex<V,E,D>,
        }
    }
}
impl<V: NodeT, E: EdgeT> Edge<V, E, Dir<V,E>> { }
impl<V: NodeT, E: EdgeT> Edge<V, E, Undir<V,E>> { }

///////////////////////////////////////////////////////////////////////////////
//  Graph
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Graph<V: NodeT, E: EdgeT, D: DirT<E>> {
    nodes: HashMap<&'static V, Vertex<V,E,D>>,
    edges: Vec<Box<Edge<V,E,D>>>,
}

impl<V: NodeT, E: EdgeT, D: DirT<E>> Graph<V,E,D> {
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: Vec::new(), } }
    pub fn with_capacity(n: usize, m: usize) -> Self {
        Graph { nodes: HashMap::with_capacity(n), edges: Vec::with_capacity(m) }
    }
    pub fn insert_vertex(&mut self, v: V) {
        assert!(self.nodes.contains_key(&v) == false);
        let vert = Vertex::new(v);
        let val = unsafe {
            mem::transmute::<&V, &'static V>(vert.borrow())
        };
        self.nodes.insert(val, vert);
    }
    fn get_vertex(&self, v: &V) -> Option<&Vertex<V,E,D>> {
        self.nodes.get(v)
    }
    fn create_edge(&mut self, e: E, l: &V, r: &V) -> Option<*const Edge<V,E,D>> {
        let edge = {
            let lhs = self.get_vertex(l)?;
            let rhs = self.get_vertex(r)?;
            Edge::new(e, lhs, rhs)
        };
        self.edges.push(Box::new(edge));
        let edge: &Edge<V,E,D> = &self.edges.last().unwrap();
        Some(edge as *const Edge<V,E,D>)
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Undir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let ptr = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.hood.neighbors.push(ptr);
        self.nodes.get_mut(r)?.hood.neighbors.push(ptr);
        Some(())
    }
}

impl<V: NodeT, E: EdgeT> Graph<V, E, Dir<V,E>> {
    pub fn insert_edge(&mut self, e: E, l: &V, r: &V) -> Option<()> {
        let ptr = self.create_edge(e, l, r)?;
        self.nodes.get_mut(l)?.hood.children.push(ptr);
        self.nodes.get_mut(r)?.hood.parents.push(ptr);
        Some(())
    }
}

