use competitive_tools_rust::io::{parse_line, parse_tuple2};

fn main() {
    let n: usize = parse_line();
    let (mut x, mut y): (Vec<isize>, Vec<isize>) = (0..n).fold((vec![], vec![]), |mut acc, _| {
        let (x_item, y_item): (isize, isize) = parse_tuple2();
        acc.0.push(x_item);
        acc.1.push(y_item);
        acc
    });
    println!("{}", min_inconvenient(&mut x) + min_inconvenient(&mut y));
}

fn min_inconvenient(v: &mut [isize]) -> usize {
    v.sort_unstable();
    let best_pos = v[v.len() / 2];
    v.iter()
        .fold(0, |acc, v_item| acc + ((best_pos - v_item).abs() as usize))
}
