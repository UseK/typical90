use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let s: Vec<char> = parse_line::<String>().chars().collect();
    let mut dp: Vec<Vec<String>> = vec![vec!["".to_string(); n]; k];
    let mut current_min = s[0].to_string();
    for j in 0..n {
        if s[j].to_string() < current_min {
            current_min = s[j].to_string();
        }
        dp[0][j] = current_min.clone();
    }

    for i in 1..k {
        let mut prev = dp[i - 1][i - 1].clone();
        //println!("{}", prev);
        prev.push(s[i]);
        //println!("{}", prev);
        dp[i][i] = prev;
        for j in i + 1..n {
            let mut added = dp[i - 1][j].clone();
            added.push(s[j]);
            if added < dp[i][j - 1] {
                dp[i][j] = added;
            } else {
                dp[i][j] = dp[i][j - 1].clone();
            }
        }
    }
    dp.iter().for_each(|item| println!("{:?}", item));
}
