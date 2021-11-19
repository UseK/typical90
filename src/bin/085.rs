use std::collections::BTreeSet;

use competitive_tools_rust::{io::parse_line, math::prime_factors};

fn main() {
    let k: usize = parse_line();
    let factors = prime_factors(k);
    // d!(factors);

    let mut answers = BTreeSet::new();

    for i in 0..3usize.pow(factors.len() as u32) {
        let abc_ind = number_system(i, 3, factors.len());
        // d!(i, abc_ind);
        let mut abc = vec![1; 3];
        for j in 0..factors.len() {
            abc[abc_ind[j]] *= factors[j];
        }
        if abc[0] <= abc[1] && abc[1] <= abc[2] {
            answers.insert(abc);
        }
    }
    println!("{}", answers.len());
}

fn number_system(n: usize, radix: usize, length: usize) -> Vec<usize> {
    let mut n = n;
    (0..length)
        .map(|_| {
            let x = n % radix;
            n /= radix;
            x
        })
        .collect()
}
