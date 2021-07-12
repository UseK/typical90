use competitive_tools_rust::io::{parse_tuple3, parse_values};
use itertools::Itertools;

fn main() {
    let (n, p, q): (usize, usize, usize) = parse_tuple3();
    let a: Vec<usize> = parse_values(n);
    let ans = a
        .iter()
        .combinations(5)
        .filter(|pattern| {
            let remainder = pattern.iter().fold(1, |acc, &&p_item| (acc * p_item) % p);
            remainder == q
        })
        .count();
    println!("{}", ans);
}
