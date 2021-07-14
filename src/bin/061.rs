use competitive_tools_rust::io::{parse_line, parse_tuple2};
use std::collections::VecDeque;

fn main() {
    let q: usize = parse_line();
    let mut queue: VecDeque<usize> = VecDeque::new();
    for _ in 0..q {
        let (t, x): (usize, usize) = parse_tuple2();
        match t {
            1 => queue.push_front(x),
            2 => queue.push_back(x),
            3 => {
                println!("{}", queue[x - 1]);
            }
            _ => {}
        }
    }
}
