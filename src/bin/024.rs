use competitive_tools_rust::{io::*, math::AbsDiff};

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let a: Vec<usize> = parse_values(n);
    let b: Vec<usize> = parse_values(n);
    let diff_sum = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (x, y)| acc + x.abs_diff(*y));
    println!(
        "{}",
        if diff_sum <= k && (k - diff_sum) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
