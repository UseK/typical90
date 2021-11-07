use competitive_tools_rust::io::parse_tuple2;
use competitive_tools_rust::math::ModDiv;

const LAW: usize = 1_000_000_007;

fn main() {
    let (left, right): (usize, usize) = parse_tuple2();
    let left_d = count_digit(left);
    let right_d = count_digit(right);
    // d!(left, right);
    // d!(left_d, right_d);
    let ans = (left_d..=right_d).fold(0, |acc, d| {
        // d!(acc);
        let d_min = 10usize.pow((d - 1) as u32);
        let d_max = 10usize.pow(d as u32) - 1;
        // d!(d, d_min, d_max);
        (acc + d * arithmetic_progression_1(left.max(d_min), right.min(d_max))) % LAW
    });
    println!("{}", ans);
}

fn arithmetic_progression_1(left: usize, right: usize) -> usize {
    // d!(left, right);
    let a_raw = left + right;
    let b_raw = right - left + 1;
    a_raw.mod_div(2, LAW) * (b_raw % LAW) % LAW
}

#[allow(dead_code)]
fn naive(left: usize, right: usize) -> usize {
    (left..=right).fold(0, |acc, i| (acc + i * count_digit(i)) % LAW)
}

fn count_digit(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}
