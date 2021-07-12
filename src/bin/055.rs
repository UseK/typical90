use competitive_tools_rust::io::{parse_tuple3, parse_values};

fn main() {
    let (n, p, q): (usize, usize, usize) = parse_tuple3();
    let a: Vec<usize> = parse_values(n);
    let mut ans = 0;

    for i in 0..n {
        for j in 0..i {
            let remain_1 = (a[i] * a[j]) % p;
            for k in 0..j {
                let remain_2 = (remain_1 * a[k]) % p;
                for l in 0..k {
                    let remain_3 = (remain_2 * a[l]) % p;
                    for m in 0..l {
                        if (remain_3 * a[m]) % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
