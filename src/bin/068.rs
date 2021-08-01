// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_tuple4};

fn main() {
    let n: usize = parse_line();
    let mut sums: Vec<Option<usize>> = vec![None; n + 1];
    let q: usize = parse_line();
    for _ in 0..q {
        let (t, x, y, v): (usize, usize, usize, usize) = parse_tuple4();
        if t == 0 {
            sums[x] = Some(v);
            // d!(sums);
        } else {
            // d!(x, y, v);
            if x < y {
                operate(&sums, x..y, v);
            } else {
                // d!("reverse!!!");
                operate(&sums, (y..x).rev(), v);
            }
        }
        // d!("--------");
    }
}

fn operate<R>(sums: &[Option<usize>], r: R, v: usize)
where
    R: Iterator<Item = usize>,
{
    let mut current: isize = v as isize;
    for i in r {
        if let Some(sum) = sums[i] {
            current = sum as isize - current;
            // d!(i, sums[i], current);
        } else {
            println!("Ambiguous");
            return;
        }
    }
    println!("{}", current);
}
