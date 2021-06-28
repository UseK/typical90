use competitive_tools_rust::graph::BipartiteGraph;
use competitive_tools_rust::io::*;

fn ans(list: Vec<usize>, n: usize) {
    println!(
        "{}",
        list.iter()
            .take(n / 2)
            .map(|i| (i + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let n: usize = parse_line();
    let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = parse_tuple2();
        adjacency_list[a - 1].push(b - 1);
        adjacency_list[b - 1].push(a - 1);
    }
    let (x, y) = adjacency_list.bi_partition();
    if x.len() <= y.len() {
        ans(y, n);
    } else {
        ans(x, n);
    }
}
