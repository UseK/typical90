// use competitive_tools_rust::d;
use competitive_tools_rust::io::parse_tuple2;

const LAW: usize = 1_000_000_007;

fn main() {
    let (left, right): (usize, usize) = parse_tuple2();
    let left_d = count_digit(left);
    let right_d = count_digit(right);
    // d!(left_d, right_d);
    let ans = if left_d == right_d {
        let sum = arithmetic_progression_1(left, right);
        // d!(sum, sum * count_digit(left) % LAW);
        sum * left_d % LAW
    } else {
        let left_remain = left_d * arithmetic_progression_1(left, max_in_the_digit(left));
        let right_remain = right_d * arithmetic_progression_1(min_in_the_digit(right), right);
        let mid_remain = (left_d + 1..right_d).fold(0, |acc, mid_d| {
            let min_in_mid_d = 10usize.pow((mid_d - 1) as u32);
            let max_in_mid_d = 10usize.pow(mid_d as u32) - 1;
            acc + mid_d * arithmetic_progression_1(min_in_mid_d, max_in_mid_d)
        });
        // d!(left_remain, right_remain, mid_remain);
        (left_remain + right_remain + mid_remain) % LAW
    };
    println!("{}", ans);
}

fn arithmetic_progression_1(left: usize, right: usize) -> usize {
    // d!(left, right);
    let a = (left % LAW + right % LAW) % LAW;
    let b = (right - left + 1) % LAW;
    // d!(a, b, ((a * b) / 2) % LAW);
    ((a * b) / 2) % LAW
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

fn max_in_the_digit(n: usize) -> usize {
    let n_digits = count_digit(n);
    10usize.pow(n_digits as u32) - 1
}

fn min_in_the_digit(n: usize) -> usize {
    10usize.pow((count_digit(n) - 1) as u32)
}

#[allow(dead_code)]
fn min_in_next_digit(n: usize) -> usize {
    10usize.pow(count_digit(n) as u32)
}
