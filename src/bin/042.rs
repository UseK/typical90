use competitive_tools_rust::io::parse_line;

const LAW: usize = 10usize.pow(9) + 7;

fn main() {
    let k: usize = parse_line();

    if k % 9 != 0 {
        println!("{}", 0);
        return;
    }

    let mut dp: Vec<usize> = vec![0; k + 10];
    dp[0] = 1;
    for i in 0..k {
        for j in 1..=9 {
            dp[i + j] = (dp[i + j] + dp[i]) % LAW;
        }
    }
    println!("{}", dp[k]);
}
