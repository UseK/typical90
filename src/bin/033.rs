use competitive_tools_rust::io::parse_tuple2;

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
}
