use competitive_tools_rust::io::*;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge {
    pub to: usize,
    pub cost: usize,
}

/// (distance, from_path)
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pair(usize, usize);

pub fn dijkstra(s: usize, max_v: usize, edges_list: &[Vec<Edge>]) -> Vec<Option<usize>> {
    let mut que = BinaryHeap::new();
    let mut min_dists: Vec<Option<usize>> = (0..max_v).map(|_| None).collect();
    min_dists[s] = Some(0);
    que.push(Reverse(Pair(0, s)));
    while let Some(Reverse(p)) = que.pop() {
        let v = p.1;
        if min_dists[v].unwrap() < p.0 {
            continue;
        };
        for e in &edges_list[v] {
            let candidate_dist = min_dists[v].unwrap() + e.cost;
            if candidate_dist < min_dists[e.to].unwrap_or(std::usize::MAX) {
                min_dists[e.to] = Some(candidate_dist);
                que.push(Reverse(Pair(candidate_dist, e.to)));
            }
        }
    }
    min_dists
}

fn main() {
    let n: usize = parse_line();
    type AdjacencyList = Vec<Vec<Edge>>;
    let abs: Vec<(usize, usize)> = (0..n - 1)
        .map(|_| parse_tuple2::<usize>())
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
