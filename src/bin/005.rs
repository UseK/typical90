// use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

const LAW: usize = 1_000_000_007;

fn main() {
    let (n, b, k): (usize, usize, usize) = parse_tuple3();
    let c: Vec<usize> = parse_values(k);
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b]; n];
    for &c_item in &c {
        dp[0][c_item % b] += 1;
    }
    for i in 1..n {
        for j in 0..b {
            for &c_item in &c {
                let new_mod = (j * 10 + c_item) % b;
                // d!(i, j, c_item, new_mod, dp[i - 1][j]);
                dp[i][new_mod] += dp[i - 1][j];
                dp[i][new_mod] %= LAW;
            }
        }
    }
    // dp.iter().for_each(|item| println!("{:?}", item));
    println!("{}", dp[n - 1][0]);
}
