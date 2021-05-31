use competitive_tools_rust::io::*;

fn main() {
    let n: usize = parse_line();
    let mut acc_one: Vec<usize> = vec![0; n + 1];
    let mut acc_two: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        let (c, p): (usize, usize) = parse_tuple2();
        if c == 1 {
            acc_one[i] = acc_one[i - 1] + p;
            acc_two[i] = acc_two[i - 1];
        } else {
            acc_one[i] = acc_one[i - 1];
            acc_two[i] = acc_two[i - 1] + p;
        }
    }

    let q: usize = parse_line();
    for _ in 0..q {
        let (a, b): (usize, usize) = parse_tuple2();
        let ans_one = acc_one[b] - acc_one[a - 1];
        let ans_two = acc_two[b] - acc_two[a - 1];
        println!("{} {}", ans_one, ans_two);
    }
}
