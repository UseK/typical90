use competitive_tools_rust::io::parse_tuple2;
use competitive_tools_rust::math::ModPow;

const LAW: usize = 10usize.pow(9) + 7;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();

    let corner = match (n, k) {
        (1, k) => Some(k),
        (_, 1) => Some(0),
        (2, 2) => Some(2),
        (_, 2) => Some(0),
        (_, _) => None,
    };
    if let Some(x) = corner {
        println!("{}", x);
        return;
    }

    println!("{}", k * (k - 1) % LAW * (k - 2).mod_pow(n - 2, LAW) % LAW);
}
