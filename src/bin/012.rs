// use competitive_tools_rust::{d};
use competitive_tools_rust::{
    io::{parse_line, parse_tuple2},
    union_find::UnionFindTree,
};
fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let n_query: usize = parse_line();
    let mut tree = UnionFindTree::new((h + 2) * (w + 2));
    let mut already: Vec<bool> = vec![false; (h + 2) * (w + 2)];
    (0..n_query).for_each(|_| {
        let values: Vec<usize> = parse_line::<String>()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if values[0] == 1 {
            let (r, c) = (values[1], values[2]);
            let pos = r * (w + 2) + c;
            already[pos] = true;
            let up_pos = pos - (w + 2);
            let down_pos = pos + (w + 2);
            let left_pos = pos - 1;
            let right_pos = pos + 1;
            // d!(values, pos);
            for &neightbor_pos in &[up_pos, down_pos, left_pos, right_pos] {
                if already[neightbor_pos] {
                    // d!(pos, neightbor_pos);
                    tree.unite(pos, neightbor_pos);
                }
            }
        } else {
            let (ra, ca, rb, cb) = (values[1], values[2], values[3], values[4]);
            let a_pos = ra * (w + 2) + ca;
            let b_pos = rb * (w + 2) + cb;
            // d!(a_pos, b_pos, already[a_pos], already[b_pos], tree.is_same(a_pos, b_pos));
            if already[a_pos] && already[b_pos] && tree.is_same(a_pos, b_pos) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    });
    // (0..(h+2)*(w+2)).for_each(|i| println!("{}: {}", i, tree.root(i)));
}
