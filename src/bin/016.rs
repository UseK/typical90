use competitive_tools_rust::io::{parse_line, parse_values};

fn main() {
    let n: usize = parse_line();
    let mut vs: Vec<usize> = parse_values(3);
    vs.sort_unstable();
    let (low, mid, high) = (vs[0], vs[1], vs[2]);

    if n % high == 0 {
        println!("{}", n / high);
        return;
    }

    let mut ans = 9999;

    for n_high in 0..=(n / high).min(9999) {
        let remain_high = n - n_high * high;
        // d!(n_high, remain_high);
        if remain_high % mid == 0 {
            // println!("{}", n_high + remain_high / mid);
            ans = ans.min(n_high + remain_high / mid);
        }
        for n_mid in 0..=(remain_high / mid).min(9999) {
            let remain_mid = remain_high - n_mid * mid;
            // d!(n_mid, remain_mid);
            if remain_mid % low == 0 {
                // d!(n_high, n_mid, remain_mid / low);
                // println!("{}", n_high + n_mid + remain_mid / low);
                ans = ans.min(n_high + n_mid + remain_mid / low);
            }
        }
    }
    println!("{}", ans);
}
