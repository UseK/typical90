use competitive_tools_rust::io::parse_tuple2;

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let ans = match (h, w) {
        (1, w) => w,
        (h, 1) => h,
        (h, w) => ((h + 1) / 2) * ((w + 1) / 2),
    };
    println!("{}", ans);
}
