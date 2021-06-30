// use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let n: usize = parse_line();
    let size: usize = 1_001;
    let mut imos: Vec<Vec<isize>> = vec![vec![0; size]; size];
    for _ in 0..n {
        let (lx, ly, rx, ry): (usize, usize, usize, usize) = parse_tuple4();
        // d!(lx, ly, rx, ry);

        imos[ly][lx] += 1;
        imos[ly][rx] -= 1;
        imos[ry][lx] -= 1;
        imos[ry][rx] += 1;
        // imos.iter().for_each(|item| println!("{:?}", item));
        // println!();
    }
    // imos.iter().for_each(|item| println!("{:?}", item));
    // println!();
    imos_operate(&mut imos);
    let mut answers: Vec<usize> = vec![0; n + 1];
    for y in 0..imos.len() {
        for x in 0..imos[0].len() {
            answers[(imos[y][x]) as usize] += 1;
        }
    }
    answers
        .iter()
        .skip(1)
        .for_each(|item| println!("{:?}", item));
}

fn imos_operate(imos: &mut Vec<Vec<isize>>) {
    for y in 0..imos.len() {
        let mut current = 0;
        for x in 0..imos[0].len() {
            imos[y][x] += current;
            current = imos[y][x];
        }
    }
    // imos.iter().for_each(|item| println!("{:?}", item));
    // println!();

    for x in 0..imos[0].len() {
        let mut current = 0;
        for y in 0..imos.len() {
            imos[y][x] += current;
            current = imos[y][x];
        }
    }
    // imos.iter().for_each(|item| println!("{:?}", item));
}
