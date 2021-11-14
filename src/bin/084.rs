// use competitive_tools_rust::d;
use competitive_tools_rust::{io::parse_line, my_itertools::run_length_encoding};

fn main() {
    let n: usize = parse_line();
    let s: String = parse_line();
    let rle = run_length_encoding(&s.chars().collect::<Vec<char>>());
    // d!(rle);
    let ans = rle
        .iter()
        .fold(n * (n + 1) / 2, |sum, (_c, i)| sum - (i * (i + 1) / 2));
    println!("{}", ans);
}
