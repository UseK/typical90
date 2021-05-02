use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use std::iter::once;

fn main() {
    let (n, l): (usize, usize) = parse_tuple2();
    let k: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    for i in 0..l {
        d!(i, judge(i, k, l, &a));
    }
}

fn judge(score: usize, k: usize, l: usize, a: &[usize]) -> bool {
    let mut count = 0;
    let mut current_pos = 0;
    for i in 0..a.len() {
        if a[i] - current_pos >= score {
            current_pos = a[i];
            count += 1;
            //d!(i, current_pos, count);
            if count == k {
                return l - current_pos >= score;
            }
        }
    }
    false
}
