//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use std::f64::consts::PI;

fn main() {
    let t: usize = parse_line();
    let (l, x, y): (f64, f64, f64) = parse_tuple3();
    //d!(t, l, x, y);
    let q: usize = parse_line();
    let e: Vec<usize> = (0..q).map(|_| parse_line()).collect();
    //e.iter().for_each(|item| println!("{:?}", item));
    e.iter().for_each(|e_item| {
        let period: f64 = (e_item % t) as f64 / t as f64;
        let theta: f64 = 2.0 * PI * period;
        let current_y: f64 = -0.5 * l * theta.sin();
        let current_z: f64 = 0.5 * l * (1.0 - theta.cos());
        //d!(period, theta, current_y, current_z);
        let xy_diff = ((current_y - y) * (current_y - y) + x * x).sqrt();
        //d!(xy_diff);
        let ans = current_z.atan2(xy_diff) / PI * 180.0;
        println!("{}", ans);
    });
}
