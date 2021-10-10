// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_values};

fn main() {
    let n: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    let whole_size: usize = a.iter().sum();
    // d!(whole_size);
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut sum: usize = 0;
    loop {
        // d!(start, end, sum * 10);
        match (sum * 10).cmp(&whole_size) {
            std::cmp::Ordering::Equal => {
                println!("Yes");
                return;
            }
            std::cmp::Ordering::Less => {
                // d!("Less!");
                sum += a[end];
                end += 1;
                if end >= n {
                    end = 0;
                }
            }
            std::cmp::Ordering::Greater => {
                // d!("Greater!");
                sum -= a[start];
                start += 1;
                if start >= n {
                    println!("No");
                    return;
                }
            }
        }
    }
}
