
use std::collections::HashSet;
use std::borrow::Borrow;

use super::{DiGraph, Graph, UnweightedUndirectedGraph};
use super::UnweightedEdge;

// poset lattice looking tree thing
// edge from 0 to 1..9
// edge from 1 to 10..19
// edge from 2 to 20..29
// etc.
fn numerical_tree(n: usize) -> UnweightedUndirectedGraph<usize> {
    let mut g = Graph::new();
    g.insert_vertex(0);
    for i in 1..n {
        g.insert_vertex(i);
        let res = g.insert_undirected_edge(UnweightedEdge, &(i/10), &i);
        assert!(res.is_some());
    }
    g
}

#[test]
fn insert() {
    let mut g = DiGraph::<char, u8>::new();
    g.insert_vertex('A');
    g.insert_vertex('B');
    g.insert_vertex('C');
    g.insert_directed_edge(1, &'A', &'B');
    g.insert_directed_edge(2, &'B', &'C');
    g.insert_directed_edge(3, &'C', &'A');
}

#[test]
fn comprehensive_iter() {
    const TREE_SIZE: usize = 100;
    let g = numerical_tree(TREE_SIZE);
    let mut seen: HashSet<usize> = HashSet::new();
    for i in g.vertices() {
        let n: &usize = i.borrow();
        assert!(seen.contains(n) == false);
        seen.insert(*n);
    }
    assert_eq!(TREE_SIZE, seen.len());
    assert_eq!(TREE_SIZE, g.order());
    assert_eq!(TREE_SIZE-1, g.size());
}

#[test]
fn neighborhood_sizes() {
    let g = numerical_tree(100);
    for i in g.vertices() {
        let degree = match *i.borrow() {
            0       => 9,       // edge to 0..9
            1...9   => 1 + 10,  // edge to 0 and n0..n9
            10...99 => 1,       // edge to range(00,90,10)
            _ => unreachable!(),
        };
        assert_eq!(i.degree(), degree, "Failed at node {:?}", i);
    }
}

#[test]
fn breadth_first() {
    let g = numerical_tree(100);
    let start = g.get_vertex(&42).unwrap();
    assert_eq!(100, g.breadth_first(Some(start)).count());
    assert_eq!(100, g.breadth_first(None).count());

    let root = g.get_vertex(&0).unwrap();
    for (i,n) in g.breadth_first(Some(root)).enumerate() {
        let x: usize = *n.get();
        match i { 
            0       => assert_eq!(x, 0, "started at wrong node"),
            1 ... 9 => { assert!(x > 0); assert!(x < 10) },
            10...99 => { assert!(x > 9); assert!(x < 100) },
            _ => unreachable!(),
        }
    }
}

/*
#[test]
fn foo() {
    let g = numerical_tree(100);
    let i = g.get_vertex(&3).unwrap();
    let ii: &usize = i.borrow();
    print!("Node `{}` neighbors:  ", *ii);
    for n in g.get_neighbors(i) {
        let nn: &usize = n.borrow();
        print!("{},", *nn);
    }
    println!();
    println!("AAAAAAAAAAAAAAAAA");

}

*/
