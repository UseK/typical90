//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use competitive_tools_rust::search::bound;

fn main() {
    let n: usize = parse_line();
    let mut a: Vec<usize> = parse_values(n);
    a.sort_unstable();
    let q: usize = parse_line();

    for _ in 0..q {
        let b: usize = parse_line();
        let (l, r) = bound(-1, a.len() as isize, |i| b <= a[i as usize]);
        //d!(b, l, r);
        let ans = if l.is_negative() {
            a[r as usize] - b
        } else if r as usize == a.len() {
            b - a[l as usize]
        } else {
            (a[r as usize] - b).min(b - a[l as usize])
        };
        println!("{:?}", ans);
    }
}
