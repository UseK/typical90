use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_tuple4};
use competitive_tools_rust::union_find::UnionFindTree;

fn main() {
    let n: usize = parse_line();
    let q: usize = parse_line();
    let txyv: Vec<(usize, usize, usize, isize)> = (0..q).map(|_| parse_tuple4()).collect();
    let mut sums: Vec<Option<isize>> = vec![None; n + 1];
    for &(t, x, _y, v) in &txyv {
        if t == 0 {
            sums[x] = Some(v);
        }
    }
    d!(sums);
    let mut acc_sums: Vec<Option<isize>> = vec![None; n + 1];
    acc_sums[0] = Some(0);
    for i in 1..=n {
        if let Some(acc_previous) = acc_sums[i - 1] {
            if let Some(sum_current) = sums[i] {
                acc_sums[i] = Some(sum_current - acc_previous);
            }
        }
    }
    d!(acc_sums);

    let mut union_find = UnionFindTree::new(n + 1);
    for &(t, x, y, v) in &txyv {
        if t == 0 {
            union_find.unite(x, y);
        } else if union_find.is_same(x, y) {
            d!(x, y, v);
        } else {
            println!("Ambiguous");
        }
    }

    // union_find.unite(x, y);
    // for _ in 0..q {
    //     let (t, x, y, v): (usize, usize, usize, usize) = parse_tuple4();
    //     if t == 0 {
    //         sums[x] = Some(v);
    //         // d!(sums);
    //     } else {
    //         // d!(x, y, v);
    //         if x < y {
    //             operate(&sums, x..y, v);
    //         } else {
    //             // d!("reverse!!!");
    //             operate(&sums, (y..x).rev(), v);
    //         }
    //     }
    //     // d!("--------");
    // }
}

// fn operate<R>(sums: &[Option<usize>], r: R, v: usize)
// where
//     R: Iterator<Item = usize>,
// {
//     let mut current: isize = v as isize;
//     for i in r {
//         if let Some(sum) = sums[i] {
//             current = sum as isize - current;
//             // d!(i, sums[i], current);
//         } else {
//             println!("Ambiguous");
//             return;
//         }
//     }
//     println!("{}", current);
// }
