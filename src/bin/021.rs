use competitive_tools_rust::graph::AdjacencyList;
use competitive_tools_rust::io::*;

fn main() {
    let (n, m): (usize, usize) = parse_tuple2();
    let ab: Vec<(usize, usize)> = (0..m).map(|_| parse_tuple2()).collect();
    let adjacency_list: Vec<Vec<usize>> = AdjacencyList::from_atcoder_tuples(n, &ab);
    let scc_order = adjacency_list.strongly_connected_component();
    let mut same_orders_count: Vec<usize> = vec![0; n];
    for order_id in scc_order {
        same_orders_count[order_id] += 1;
    }
    let ans = same_orders_count
        .into_iter()
        .fold(0, |acc, count| acc + combination(count, 2));

    println!("{:?}", ans);
}

fn combination(n: usize, r: usize) -> usize {
    if n < r {
        0
    } else {
        let numerator: usize = ((n - r + 1)..=n).product();
        let denominator: usize = (1..=r).product();
        numerator / denominator
    }
}
