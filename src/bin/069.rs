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

    let mut acc = k * (k - 1);
    let k_minus_2 = k - 2;
    for _ in 2..n {
        acc = (acc * k_minus_2) % LAW;
    }
    println!("{}", acc);
}
