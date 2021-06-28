use competitive_tools_rust::io::*;
use std::collections::HashSet;

fn main() {
    let n: usize = parse_line();
    let mut s = HashSet::new();
    for i in 0..n {
        if s.insert(parse_line::<String>()) {
            println!("{}", i + 1);
        }
    }
}
