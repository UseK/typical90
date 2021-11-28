use std::iter::once;

use competitive_tools_rust::{
    // d,
    io::{parse_line, parse_tuple2, parse_values},
    search::bound,
};
use itertools::Itertools;

fn main() {
    let (n, l): (usize, usize) = parse_tuple2();
    let k: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    let pieces: Vec<usize> = once(0)
        .chain(a)
        .chain(once(l))
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect();
    // d!(pieces);
    let (_ng_bound, ok_bound) = bound(l as isize, 1, |min_length| {
        let min_length = min_length as usize;
        let mut acc_length = 0;
        let mut count = 0;
        pieces.iter().for_each(|&a_item| {
            acc_length += a_item;
            if acc_length >= min_length {
                count += 1;
                acc_length = 0;
            }
        });
        // d!(min_length, count, acc_length);
        // d!(count > k);
        count > k
    });
    println!("{}", ok_bound);
}
