use competitive_tools_rust::io::*;
use competitive_tools_rust::search::bound;
use itertools::Itertools;
use std::f64::consts::PI;

fn main() {
    let n: usize = parse_line();
    let xy: Vec<(f64, f64)> = (0..n).map(|_| parse_tuple2()).collect();
    // xy.iter().for_each(|item| println!("{:?}", item));

    let mut max_ans: f64 = 0.0;
    for i in 0..n {
        // println!("---");
        let base = xy[i];
        let relative_xy: Vec<(f64, f64)> = xy
            .iter()
            .enumerate()
            .filter_map(|(ind, (x, y))| {
                if ind == i {
                    None
                } else {
                    Some((x - base.0, y - base.1))
                }
            })
            .collect();

        // relative_xy.iter().for_each(|item| println!("{:?}", item));
        let atan_sorted: Vec<f64> = relative_xy
            .iter()
            .map(|(x, y)| mod_360(y.atan2(*x) / PI * 180.0))
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect();
        // atan_sorted.iter().for_each(|item| println!("{:?}", item));
        let ans = atan_sorted
            .iter()
            .enumerate()
            .map(|(ind, item)| {
                let far_point = mod_180(if item > &0.0 {
                    item - 180.0
                } else {
                    item + 180.0
                });
                // d!(far_point);
                let (left, right) = bound(0, (atan_sorted.len() - 1) as isize, |i| {
                    far_point <= atan_sorted[i as usize]
                });
                // d!(atan_sorted[left as usize]);
                // d!(atan_sorted[right as usize]);
                let left_v = if left as usize == ind {
                    0.0
                } else {
                    (item - atan_sorted[left as usize]).abs()
                };
                let right_v = if right as usize == ind {
                    0.0
                } else {
                    (item - atan_sorted[right as usize]).abs()
                };
                // d!(left_v);
                // d!(right_v);
                mod_180(left_v.max(right_v))
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        max_ans = max_ans.max(ans);
    }
    println!("{}", max_ans);
}

fn mod_360(v: f64) -> f64 {
    if v < 0.0 {
        v + 360.0
    } else {
        v
    }
}

fn mod_180(v: f64) -> f64 {
    if v < 0.0 {
        v + 180.0
    } else if v > 180.0 {
        v - 180.0
    } else {
        v
    }
}
