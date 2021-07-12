use competitive_tools_rust::io::{parse_tuple3, parse_values};
use itertools::Itertools;

fn main() {
    let (n, p, q): (usize, usize, usize) = parse_tuple3();
    let a: Vec<usize> = parse_values(n);
    let mut ans = 0;
    a.into_iter().combinations(5).for_each(|pattern| {
        let remainder = pattern.into_iter().reduce(|acc, p_item| (acc * p_item) % p);
        if remainder.unwrap() == q {
            ans += 1;
        }
    });
    println!("{}", ans);
}
