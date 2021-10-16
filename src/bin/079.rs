// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_tuple2, parse_values};

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let mut matrix_start: Vec<Vec<isize>> = (0..h).map(|_| parse_values(w)).collect();

    let matrix_goal: Vec<Vec<isize>> = (0..h).map(|_| parse_values(w)).collect();

    let mut diff_sum: usize = 0;

    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let diff = matrix_goal[y][x] - matrix_start[y][x];
            // d!(x, y, diff);
            diff_sum += diff.abs() as usize;
            matrix_start[y][x] += diff;
            matrix_start[y][x + 1] += diff;
            matrix_start[y + 1][x] += diff;
            matrix_start[y + 1][x + 1] += diff;
            // matrix_start.iter().for_each(|item| d!(item));
        }
    }
    if matrix_start == matrix_goal {
        println!("Yes");
        println!("{}", diff_sum);
    } else {
        println!("No");
    }
}
