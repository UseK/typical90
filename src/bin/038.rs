use competitive_tools_rust::{io::parse_tuple2, math::Gcd};

fn main() {
    let (a, b): (usize, usize) = parse_tuple2();
    let gcd = a.gcd(b) as u128;
    let ans: u128 = (a as u128) / gcd * (b as u128);
    if ans > 10u128.pow(18) {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
