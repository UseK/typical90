use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use itertools::Itertools;
use std::f64::consts::PI;

fn main() {
    let n: usize = parse_line();
    let xy: Vec<(f64, f64)> = (0..n).map(|_| parse_tuple2()).collect();
    xy.iter().for_each(|item| println!("{:?}", item));

    for i in 0..n {
        println!("---");
        let base = xy[i];
        let relative_xy: Vec<(f64, f64)> =
            xy.iter().map(|(x, y)| (x - base.0, y - base.1)).collect();
        relative_xy.iter().for_each(|item| println!("{:?}", item));
        let atan_sorted: Vec<f64> = relative_xy
            .iter()
            .map(|(x, y)| y.atan2(*x))
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect();
        atan_sorted.iter().for_each(|item| println!("{:?}", item));
    }
}
