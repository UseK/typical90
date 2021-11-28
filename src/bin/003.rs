use competitive_tools_rust::graph::{dijkstra, Edge};
use competitive_tools_rust::io::*;

fn main() {
    let n: usize = parse_line();
    type AdjacencyList = Vec<Vec<Edge>>;
    let abs: Vec<(usize, usize)> = (0..n - 1)
        .map(|_| parse_tuple2::<usize, usize>())
        .map(|(a, b)| (a - 1, b - 1))
        .collect();
    let edges_list: AdjacencyList = abs.into_iter().fold(vec![vec![]; n], |mut acc, (a, b)| {
        acc[a].push(Edge { to: b, cost: 1 });
        acc[b].push(Edge { to: a, cost: 1 });
        acc
    });
    //edges_list.iter().for_each(|item| println!("{:?}", item));
    let (max_ind, _) = dijkstra(0, n, &edges_list)
        .into_iter()
        .flatten()
        .enumerate()
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    //println!("{:?}", max_ind);
    debug_assert!(dijkstra(max_ind, n, &edges_list)
        .iter()
        .all(|v| v.is_some()));
    let ans = dijkstra(max_ind, n, &edges_list)
        .iter()
        .flatten()
        .max()
        .unwrap()
        + 1;
    println!("{}", ans);
}
