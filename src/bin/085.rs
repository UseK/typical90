use competitive_tools_rust::{io::parse_line};

fn main() {
    let k: usize = parse_line();
    let dvs = divisors(k);
    // d!(dvs);
    let mut ans = 0;
    for i in 0..dvs.len(){
        for j in i..dvs.len() {
            let a = dvs[i];
            let b = dvs[j];
            // d!(a, b);
            if k / a < b { continue };
            if k % (a * b) != 0 { continue };
            let c = k / (a * b);
            if c < b { continue };
            // d!(a, b, c);
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn divisors(n: usize) -> Vec<usize> {
    let mut dvs = vec![];
    for i in 1..=n {
        if i * i > n { break }
        if n % i == 0 {
            dvs.push(i);
            if i * i != n { dvs.push(n / i)}
        }
    }
    dvs.sort_unstable();
    dvs
}
