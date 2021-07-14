use competitive_tools_rust::io::{parse_line, parse_tuple2};

fn main() {
    let q: usize = parse_line();
    let mut queue: Vec<usize> = vec![];
    for _ in 0..q {
        let (t, x): (usize, usize) = parse_tuple2();
        match t {
            1 => queue.insert(0, x),
            2 => queue.push(x),
            3 => {
                println!("{}", queue[x - 1]);
            }
            _ => {}
        }
    }
}
