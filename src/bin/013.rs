use competitive_tools_rust::graph::{dijkstra, Edge};
use competitive_tools_rust::io::*;

fn main() {
    let (n, m): (usize, usize) = parse_tuple2();
    let mut edges_list = vec![vec![]; n];
    for _ in 0..m {
        let (a, b, cost): (usize, usize, usize) = parse_tuple3();
        edges_list[a - 1].push(Edge { to: b - 1, cost });
        edges_list[b - 1].push(Edge { to: a - 1, cost });
    }
    let from_start = dijkstra(0, n, &edges_list);
    // from_start.iter().for_each(|item| println!("{:?}", item));
    // println!();

    let from_goal = dijkstra(n - 1, n, &edges_list);
    // from_goal.iter().for_each(|item| println!("{:?}", item));
    from_start.iter().zip(&from_goal).for_each(|(x, y)| {
        println!("{}", x.unwrap() + y.unwrap());
    })
}
