use competitive_tools_rust::io::*;

const ATCODER: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
const LAW: usize = 1_000_000_007;

fn main() {
    let n: usize = parse_line();
    let s: Vec<char> = parse_line::<String>().chars().collect();
    //println!("{:?}", s);
    let mut dp = [
        vec![0; n],
        vec![0; n],
        vec![0; n],
        vec![0; n],
        vec![0; n],
        vec![0; n],
        vec![0; n],
    ];
    let mut current = 0;
    for i in 0..n {
        if s[i] == ATCODER[0] {
            current += 1;
        }
        dp[0][i] = current;
    }
    for i in 1..7 {
        for j in 1..n {
            dp[i][j] = if s[j] == ATCODER[i] {
                (dp[i][j - 1] + dp[i - 1][j - 1]) % LAW
            } else {
                dp[i][j - 1] % LAW
            }
        }
    }
    //dp.iter().for_each(|item| println!("{:?}", item));
    println!("{}", dp[6][n - 1]);
}
