// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_tuple2, parse_values};
use itertools::Itertools;

fn main() {
    let n: usize = parse_line();
    let a: Vec<Vec<usize>> = (0..n).map(|_| parse_values(n)).collect();
    let m: usize = parse_line();
    let xy: Vec<(usize, usize)> = (0..m).map(|_| parse_tuple2::<usize, usize>()).collect();
    let mut ng_matrix = vec![vec![false; n]; n];
    for (x, y) in xy {
        ng_matrix[x - 1][y - 1] = true;
        ng_matrix[y - 1][x - 1] = true;
    }

    // println!("{:?}", a);
    // println!();
    // println!();
    // println!("{:?}", ng_matrix);

    let mut min_score: Option<usize> = None;
    (0..n).permutations(n).for_each(|pattern| {
        // println!("{:?}", pattern);
        let is_ok = pattern
            .iter()
            .tuple_windows::<(&usize, &usize)>()
            // .inspect(|t| d!(t))
            .all(|(&a, &b)| !ng_matrix[a][b]);
        if is_ok {
            let score = pattern
                .iter()
                .enumerate()
                .fold(0, |acc, (ind, &p_item)| acc + a[p_item][ind]);
            // d!(score);
            let current = min_score.unwrap_or(score);
            min_score = Some(current.min(score));
            // d!(min_score);
        }
    });

    println!(
        "{}",
        min_score.map_or("-1".to_string(), |score| score.to_string())
    );
}
