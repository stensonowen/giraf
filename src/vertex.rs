use std::fmt;

use Addr;

pub trait Node: fmt::Debug {}

#[derive(Debug)] 
pub struct Vertex<T: Node> {
    val: T,
    addr: Addr, // is this necessary?
}

impl<T: Node> Vertex<T> {
    pub(crate) fn from(val: T, addr: Addr) -> Self {
        Vertex { val, addr }
    }
}

