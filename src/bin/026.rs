use competitive_tools_rust::io::*;

pub trait BipartiteGraph {
    fn bi_partition(&self) -> (Vec<usize>, Vec<usize>);
}

pub trait Tree: BipartiteGraph {}

impl BipartiteGraph for Vec<Vec<usize>> {
    fn bi_partition(&self) -> (Vec<usize>, Vec<usize>) {
        let mut current: Vec<Option<bool>> = vec![None; self.len()];
        let mut stack: Vec<(usize, bool)> = vec![];
        current[0] = Some(true);
        stack.append(&mut self[0].iter().map(|&i| (i, false)).collect());

        while let Some((vertex_id, bi)) = stack.pop() {
            current[vertex_id] = Some(bi);
            let nexts = self[vertex_id]
                .iter()
                .filter(|&&i| current[i].is_none())
                .map(|&i| (i, !bi));
            stack.append(&mut nexts.collect());
        }
        // println!("{:?}", current);
        let mut x: Vec<usize> = vec![];
        let mut y: Vec<usize> = vec![];
        for (ind, bi) in current.iter().enumerate() {
            if bi.unwrap() {
                x.push(ind);
            } else {
                y.push(ind);
            }
        }
        (x, y)
    }
}

impl Tree for Vec<Vec<usize>> {}

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
        // println!("{:?}", adjacency_list);
    }
    let (x, y) = adjacency_list.bi_partition();
    if x.len() <= y.len() {
        ans(y, n);
    } else {
        ans(x, n);
    }
}
