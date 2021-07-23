use competitive_tools_rust::io::parse_tuple2;

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

    let exp_part = naive_modular_exponentiation(k - 2, n - 2, LAW);
    println!("{}", k * (k - 1) * exp_part % LAW);
}

fn naive_modular_exponentiation(b: usize, e: usize, m: usize) -> usize {
    let mut exp = 1;
    for _ in 0..e {
        exp = (exp * b) % m;
    }
    exp
}
