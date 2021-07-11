use competitive_tools_rust::io::parse_tuple2;

const LAW: usize = 1_000_000_007;

fn main() {
    let (n, l): (usize, usize) = parse_tuple2();
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] = (dp[i] + dp[i - 1]) % LAW;
        if i >= l {
            dp[i] = (dp[i] + dp[i - l]) % LAW;
        }
    }
    println!("{}", dp[n]);
}
