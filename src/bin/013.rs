use competitive_tools_rust::graph::{dijkstra, Edge};
use competitive_tools_rust::io::*;

fn main() {
    let (n, m): (usize, usize) = parse_tuple2();
    let mut edges_list: Vec<Vec<Edge>> = vec![vec![]; n];
    (0..m).for_each(|_| {
        let (a, b, c): (usize, usize, usize) = parse_tuple3();
        edges_list[a - 1].push(Edge { cost: c, to: b - 1 });
        edges_list[b - 1].push(Edge { cost: c, to: a - 1 });
    });
    // edges_list.iter().for_each(|item| println!("{:?}", item));
    let dijkstra_from_start = dijkstra(0, n, &edges_list);
    // println!("{:?}", dijkstra_from_start);
    let dijkstra_from_end = dijkstra(n - 1, n, &edges_list);
    // println!("{:?}", dijkstra_from_end);

    dijkstra_from_start
        .into_iter()
        .flatten()
        .zip(dijkstra_from_end.into_iter().flatten())
        .for_each(|(a, b)| println!("{}", a + b));
}
