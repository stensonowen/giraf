
use std::collections::HashSet;
use std::borrow::Borrow;

use super::{DiGraph, Graph, UnweightedUndirectedGraph, UndirectedGraph};
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

/*
fn hypercube(dim: usize) -> UnweightedUndirectedGraph<usize> {
    let mut g = Graph::new();
    for i in 0 .. 1 << dim {
        g.insert_vertex(i);
    }
    unimplemented!();
    g
}
*/

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
fn depth_first_lattice_undir() {
    // 0, 1, 10, 11, .., 19, 2, 20, 21, .., 29, 3, ...
    let g = numerical_tree(100);
    assert_eq!(100, g.depth_first(None).count());
    let start = g.get_vertex(&0).unwrap();
    let mut df = g.depth_first(Some(start)).map(|v| *v.as_ref());
    assert_eq!(0, df.next().unwrap());
    for _tens in 1..10 {
        let x = df.next().unwrap();
        assert!(x > 0); assert!(x < 10);
        for _ones in 0..10 {
            let xx = df.next().unwrap();
            assert!(xx > 9); assert!(xx < 100);
            assert_eq!(x, xx/10);
        }
    }
    let mid = g.get_vertex(&5).unwrap();
    assert_eq!(100, g.depth_first(Some(mid)).count());
}

#[test]
fn breadth_first_lattice_undir() {
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


// https://en.wikipedia.org/wiki/File:MapGermanyGraph.svg
fn germany_wiki_map() -> UndirectedGraph<&'static str, u16> {
    let mut g = Graph::new();
    g.insert_vertex("Augsburg");
    g.insert_vertex("Erfurt");
    g.insert_vertex("Frankfurt");
    g.insert_vertex("Karlsruhe");
    g.insert_vertex("Kassel");
    g.insert_vertex("Mannheim");
    g.insert_vertex("München");
    g.insert_vertex("Nürnberg");
    g.insert_vertex("Stuttgart");
    g.insert_vertex("Würzburg");
    g.insert_undirected_edge(103, &"Nürnberg",  &"Würzburg");
    g.insert_undirected_edge(167, &"München",   &"Nürnberg");
    g.insert_undirected_edge(173, &"Frankfurt", &"Kassel");
    g.insert_undirected_edge(183, &"Nürnberg",  &"Stuttgart");
    g.insert_undirected_edge(186, &"Erfurt",    &"Würzburg");
    g.insert_undirected_edge(217, &"Frankfurt", &"Würzburg");
    g.insert_undirected_edge(250, &"Augsburg",  &"München");
    g.insert_undirected_edge(250, &"Augsburg",  &"Karlsruhe");
    g.insert_undirected_edge(502, &"Kassel",    &"München");
    g.insert_undirected_edge(80,  &"Karlsruhe", &"Mannheim");
    g.insert_undirected_edge(85,  &"Frankfurt", &"Mannheim");
    g
}

#[test]
fn bfs_germany_undir() {
    let g = germany_wiki_map();
    assert_eq!(g.order(), g.breadth_first(None).count());
    let start = g.get_vertex(&"Frankfurt").unwrap();
    // https://en.wikipedia.org/wiki/File:GermanyBFS.svg
    let rows: Vec<Vec<&'static str>> = vec![
        vec!["Frankfurt"],
        vec!["Mannheim", "Würzburg", "Kassel"],
        vec!["Karlsruhe", "Nürnberg", "Erfurt", "München"],
        vec!["Augsburg", "Stuttgart"]
    ];
    assert_eq!(g.order(), rows.iter().map(|v| v.len()).sum());
    let mut cities = g.breadth_first(Some(start));
    for row in rows {
        for _ in 0 .. row.len() {
            let city = cities.next().unwrap();
            assert!(row.contains(city.get()));
        }
    }
    assert!(cities.next().is_none());
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
