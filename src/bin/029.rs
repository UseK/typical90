use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (_w, n): (usize, usize) = parse_tuple2();
    let lr: Vec<(usize, usize)> = (0..n).map(|_| parse_tuple2()).collect();
    let (compressed_lr, compressed_w) = compress(lr);
    d!(compressed_lr);

    let (lr, w) = (compressed_lr, compressed_w);
    d!(w, compressed_w);

    let mut stack: Vec<Vec<bool>> = vec![vec![true; w], vec![false; w]];
    lr.into_iter().for_each(|(l, r)| {
        for h in (1..stack.len()).rev() {
            let is_vacant = stack[h][l - 1..r].iter().all(|&x| !x);
            let can_put = stack[h - 1][l - 1..r].iter().any(|&x| x);
            if is_vacant && can_put {
                for i in l - 1..r {
                    stack[h][i] = true;
                }
                if h == stack.len() - 1 {
                    stack.push(vec![false; w]);
                }
                println!("{}", h);
                break;
            }
        }
        // show_maze(&stack);
    });
}

fn compress(lr: Vec<(usize, usize)>) -> (Vec<(usize, usize)>, usize) {
    let values: Vec<usize> = lr.iter().fold(vec![], |mut acc, (l, r)| {
        acc.push(*l);
        acc.push(*r);
        acc
    });
    let (compression_map, w) = coordinate_compression(values);
    let compressed_lr: Vec<(usize, usize)> = lr
        .iter()
        .map(|(l, r)| (compression_map[*l].unwrap(), compression_map[*r].unwrap()))
        .collect();
    (compressed_lr, w)
}

fn coordinate_compression(mut values: Vec<usize>) -> (Vec<Option<usize>>, usize) {
    values.sort_unstable();
    values.dedup();
    let compression_map = values.iter().enumerate().fold(
        vec![None; values[values.len() - 1] + 1],
        |mut acc, (ind, &v)| {
            acc[v] = Some(ind);
            acc
        },
    );
    (compression_map, values.len())
}
