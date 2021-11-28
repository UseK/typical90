use competitive_tools_rust::io::*;
// use competitive_tools_rust::d;

const ATODER_LEN: usize = 7;
const ATCODER: [char; ATODER_LEN] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
const MOD: usize = 1_000_000_007;

fn main() {
    let n: usize = parse_line();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; ATODER_LEN];
    let s: Vec<char> = parse_line::<String>().chars().collect();
    // dp.iter().for_each(|item| println!("{:?}", item));
    // d!(s);
    (0..n).fold(0, |mut acc, i| {
        if s[i] == ATCODER[0] {
            acc += 1;
        }
        dp[0][i] = acc;
        acc
    });
    // dp.iter().for_each(|item| println!("{:?}", item));
    // println!();

    (1..ATODER_LEN).for_each(|i| {
        (i..n).for_each(|j| {
            dp[i][j] = (dp[i][j - 1] + if s[j] == ATCODER[i] {
                dp[i - 1][j - 1]
            } else {
                0
            }) % MOD;
        });
    });
    // dp.iter().for_each(|item| println!("{:?}", item));
    println!("{}", dp[ATODER_LEN - 1][n - 1]);
} 
