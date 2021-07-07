use competitive_tools_rust::{io::parse_tuple2, math::Gcd};

fn main() {
    let (a, b): (usize, usize) = parse_tuple2();
    let gcd = a.gcd(b);
    let threshold = 10usize.pow(18);
    let tmp = b / gcd;
    if a <= threshold / tmp {
        println!("{}", tmp * a);
    } else {
        println!("Large");
    }
}
