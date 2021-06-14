use competitive_tools_rust::io::*;
use competitive_tools_rust::math::AbsDiff;

fn main() {
    let n: usize = parse_line();
    let mut a: Vec<usize> = parse_values(n);
    let mut b: Vec<usize> = parse_values(n);
    a.sort_unstable();
    b.sort_unstable();
    let ans = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (a_item, b_item)| acc + a_item.abs_diff(*b_item));
    println!("{}", ans);
}
