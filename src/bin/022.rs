use competitive_tools_rust::io::*;
use competitive_tools_rust::math::Gcd;

fn main() {
    let (a, b, c): (usize, usize, usize) = parse_tuple3();
    let base = a.gcd(b).gcd(c);
    println!("{}", a / base + b / base + c / base - 3);
}
