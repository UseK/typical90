//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use competitive_tools_rust::search::bound;

fn main() {
    let (n, l): (usize, usize) = parse_tuple2();
    let k: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    let (_, ans) = bound(l as isize, 1, |i| judge(i as usize, k, l, &a));
    println!("{}", ans);
}

fn judge(score: usize, k: usize, l: usize, a: &[usize]) -> bool {
    let mut count = 0;
    let mut current_pos = 0;
    for i in 0..a.len() {
        if a[i] - current_pos >= score {
            current_pos = a[i];
            count += 1;
            //d!(i, current_pos, count);
            if count == k {
                return l - current_pos >= score;
            }
        }
    }
    false
}
