use competitive_tools_rust::io::parse_tuple2;

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
