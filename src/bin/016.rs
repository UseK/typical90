use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let n: usize = parse_line();
    let mut coins: Vec<usize> = parse_values(3);
    coins.sort_unstable();

    let less = coins[0];
    let mid = coins[1];
    let bigger = coins[2];
    d!(less, mid, bigger);

    if n % bigger == 0 {
        println!("{}", n / bigger);
        return;
    }

    let max_bigger_count = (n / bigger).min(9999);

    for bigger_count in (0..=max_bigger_count).rev() {
        d!(bigger_count);
        let remains_1 = n - bigger_count * bigger;
        d!(remains_1);
        //println!();
        if remains_1 % mid == 0 {
            println!("{}", bigger_count + remains_1 / mid);
            return;
        }

        let max_mid_count = (remains_1 / mid).min(9999);

        for mid_count in (0..=max_mid_count).rev() {
            d!(mid_count);
            let remains_2 = remains_1 - mid_count * mid;
            d!(remains_2);
            println!();

            if remains_2 % less == 0 {
                println!("{}", bigger_count + mid_count + remains_2 / less);
                return;
            }
        }
    }
}
