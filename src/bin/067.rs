use std::io;

pub fn parse_line<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn parse_values<T: std::str::FromStr>(n: usize) -> Vec<T> {
    let line = parse_line::<String>();
    let mut split = line.split_whitespace();
    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        let v = split.next().unwrap().parse::<T>().ok().unwrap();
        values.push(v);
    }
    values
}

pub fn parse_tuple2<T1, T2>() -> (T1, T2)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
{
    let line: String = parse_line();
    let mut split = line.split_whitespace();
    (
        split.next().unwrap().parse::<T1>().ok().unwrap(),
        split.next().unwrap().parse::<T2>().ok().unwrap(),
    )
}

fn main() {
    let (n, k): (String, usize) = parse_tuple2();
    let mut current_decimal = parse_octal(&n);
    for _ in 0..k {
        let base_9_vec = to_base_n_vector(current_decimal, 9);
        // d!(base_9_vec);
        let substituted: String = base_9_vec
            .into_iter()
            .map(|c| if c == '8' { '5' } else { c })
            .collect();
        current_decimal = parse_octal(&substituted);
        // d!(current_decimal);
    }
    // d!(current_decimal);
    println!(
        "{}",
        to_base_n_vector(current_decimal, 8)
            .iter()
            .collect::<String>()
    );
}

fn parse_octal(s: &str) -> usize {
    s.chars().fold(0, |acc, c| acc * 8 + (c as usize) - 48)
}

fn to_base_n_vector(i: usize, radix: usize) -> Vec<char> {
    if i == 0 {
        vec!['0']
    } else {
        let mut base_9_vector: Vec<char> = vec![];
        let mut current = i;
        while current > 0 {
            base_9_vector.push(char::from_digit((current % radix) as u32, 10).unwrap());
            current /= radix;
        }
        base_9_vector.reverse();
        base_9_vector
    }
}
