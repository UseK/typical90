use competitive_tools_rust::io::*;
use competitive_tools_rust::math::prime_factors;

fn main() {
    let n: usize = parse_line();
    let n_factors = prime_factors(n).len();
    // println!("{}", n_factors);
    for i in 0..2u32.pow(12) {
        // println!("i:{}", i);
        if 2usize.pow(i) >= n_factors {
            println!("{}", i);
            return;
        }
    }
}
