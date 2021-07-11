use competitive_tools_rust::io::{parse_line, parse_values};

const LAW: usize = 1_000_000_007;

fn main() {
    let n: usize = parse_line();
    let a: Vec<Vec<usize>> = (0..n).map(|_| parse_values(6)).collect();
    let sums: Vec<usize> = a
        .iter()
        .map(|a_line| a_line.iter().sum::<usize>())
        .collect();
    let mut ans = 1;
    for sum in sums {
        ans = (ans * sum) % LAW;
    }
    println!("{:?}", ans);
}
