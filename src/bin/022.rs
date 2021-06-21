use competitive_tools_rust::io::*;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let (a, b, c): (usize, usize, usize) = parse_tuple3();
    let base = gcd(c, gcd(a, b));
    println!("{}", a / base + b / base + c / base - 3);
}
