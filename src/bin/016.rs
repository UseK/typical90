use competitive_tools_rust::io::*;

const MAX_COUNT: usize = 9999;

fn main() {
    let n: usize = parse_line();
    let mut coins: Vec<usize> = parse_values(3);
    coins.sort_unstable();

    let less = coins[0];
    let mid = coins[1];
    let bigger = coins[2];

    let mut ans = MAX_COUNT;
    for bigger_count in 0..=MAX_COUNT {
        if bigger_count * bigger > n {
            break;
        }
        let remain_1 = n - bigger * bigger_count;
        for mid_count in 0..=MAX_COUNT {
            if mid_count * mid > remain_1 {
                break;
            }
            let remain_2 = remain_1 - mid * mid_count;
            if remain_2 % less == 0 {
                ans = ans.min(bigger_count + mid_count + remain_2 / less);
            }
        }
    }
    println!("{}", ans);
}
