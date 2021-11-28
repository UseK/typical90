use competitive_tools_rust::{io::parse_line, math::Divisors};

fn main() {
    let k: usize = parse_line();
    let dvs = k.divisors();
    // d!(dvs);
    let mut ans = 0;
    for i in 0..dvs.len() {
        for j in i..dvs.len() {
            let a = dvs[i];
            let b = dvs[j];
            // d!(a, b);
            if k / a < b {
                continue;
            };
            if k % (a * b) != 0 {
                continue;
            };
            let c = k / (a * b);
            if c < b {
                continue;
            };
            // d!(a, b, c);
            ans += 1;
        }
    }
    println!("{}", ans);
}
