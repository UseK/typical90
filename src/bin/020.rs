use competitive_tools_rust::io::*;

fn main() {
    let (a, b, c): (usize, usize, usize) = parse_tuple3();
    println!("{}", if a < c.pow(b as u32) { "Yes" } else { "No" });
}
