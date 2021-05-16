use competitive_tools_rust::io::*;

const LAW: usize = 1_000_000_007;

fn main() {
    let (n, b, k): (usize, usize, usize) = parse_tuple3();
    let c: Vec<usize> = parse_values(k);
    let mut dp: Vec<usize> = vec![0; b];
    c.iter().for_each(|&c_item| {
        dp[c_item % b] += 1;
    });

    let ans: Vec<usize> = (1..n).fold(dp, |acc, _| {
        let mut new_acc = vec![0; b];
        for j in 0..b {
            for &c_item in &c {
                let new_mod = (j * 10 + c_item) % b;
                // d!(i, j, c_item, new_mod, dp[i - 1][j]);
                new_acc[new_mod] += acc[j];
                new_acc[new_mod] %= LAW;
            }
        }
        new_acc
    });
    println!("{}", ans[0]);
}
