// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_tuple2, parse_values};
use std::collections::BTreeMap;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let a: Vec<usize> = parse_values(n);

    let (mut left, mut right): (usize, usize) = (0, 0);
    let mut counter: BTreeMap<usize, usize> = BTreeMap::new();
    *counter.entry(a[0]).or_insert(0) += 1;
    let mut current_n_types = 1;
    let mut ans: usize = 1;
    loop {
        // d!(left, right, ans, n_types, counter);
        if current_n_types <= k {
            ans = ans.max(right - left + 1);
            right += 1;
            if right >= n {
                break;
            };
            let entry = counter.entry(a[right]).or_insert(0);
            *entry += 1;
            if entry == &1 {
                current_n_types += 1;
            }
        } else {
            let entry = counter.entry(a[left]).or_insert(0);
            *entry -= 1;
            if entry == &0 {
                current_n_types -= 1;
            }
            left += 1;
        }
    }
    println!("{}", ans);
}
