use competitive_tools_rust::io::*;
use competitive_tools_rust::{search::bound};

fn main() {
    let n: usize = parse_line();
    let mut a: Vec<isize> = parse_values(n);
    a.sort_unstable();
    let q: usize = parse_line();
    let b: Vec<isize> = (0..q).map(|_| parse_line()).collect();
    // d!(n, a, q, b);
    for b_item in b {
        let (le_ind, gt_ind) = bound(-1, a.len() as isize, |i| {
            i >= a.len() as isize || i >= 0 && a[i as usize] >= b_item
        });
        // d!(b_item, le_ind, gt_ind);
        let le_diff = if le_ind.is_negative() {
            isize::MAX
        } else {
            (a[le_ind as usize] - b_item).abs()
        };
        let gt_diff = if gt_ind == a.len() as isize {
            isize::MAX
        } else {
            (a[gt_ind as usize] - b_item).abs()
        };
        println!("{}", le_diff.min(gt_diff));
    }
}
