use competitive_tools_rust::io::{parse_line, parse_values};

fn main() {
    let n: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut sum: usize = 0;
    loop {
        match (sum * 10).cmp(&n) {
            std::cmp::Ordering::Equal => {
                if sum * 10 == n {
                    println!("Yes");
                    return;
                }
            }
            std::cmp::Ordering::Less => {
                sum += a[end];
                end += 1;
                if end >= n {
                    println!("No");
                    return;
                }
            }
            std::cmp::Ordering::Greater => {
                sum -= a[start];
                start += 1;
            }
        }
    }
}
