//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let a_matrix: Vec<Vec<usize>> = (0..h).map(|_| parse_values::<usize>(w)).collect();
    //a_matrix.iter().for_each(|item| println!("{:?}", item));
    let horizontal_sums: Vec<usize> = a_matrix.iter().map(|row| row.iter().sum()).collect();
    //println!("{:?}", horizontal_sums);
    let vertical_sums: Vec<usize> = a_matrix.iter().fold(vec![0; w], |mut acc, row| {
        for i in 0..w {
            acc[i] += row[i];
        }
        acc
    });
    //d!(vertical_sums);
    for i in 0..h {
        let ans: Vec<usize> = (0..w)
            .map(|j| horizontal_sums[i] + vertical_sums[j] - a_matrix[i][j])
            .collect();
        println!(
            "{}",
            ans.iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
