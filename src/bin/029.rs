use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (_w, n): (usize, usize) = parse_tuple2();
    let lr: Vec<(usize, usize)> = (0..n).map(|_| parse_tuple2()).collect();
    let compressed_lr = compress(lr);
    d!(compressed_lr);
}

fn compress(lr: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    d!(lr);
    let values: Vec<usize> = lr.iter().fold(vec![], |mut acc, (l, r)| {
        acc.push(*l);
        acc.push(*r);
        acc
    });
    let compression_map = coordinate_compression(values);
    let compressed_lr: Vec<(usize, usize)> = lr
        .iter()
        .map(|(l, r)| (compression_map[*l].unwrap(), compression_map[*r].unwrap()))
        .collect();
    compressed_lr
}

fn coordinate_compression(mut values: Vec<usize>) -> Vec<Option<usize>> {
    values.sort_unstable();
    values.iter().enumerate().fold(
        vec![None; values[values.len() - 1] + 1],
        |mut acc, (ind, &v)| {
            acc[v] = Some(ind);
            acc
        },
    )
}
