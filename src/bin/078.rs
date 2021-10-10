use competitive_tools_rust::io::parse_tuple2;

fn main() {
    let (n, m): (usize, usize) = parse_tuple2();
    let mut less_vertex_counter: Vec<usize> = vec![0; n];
    for _ in 0..m {
        let (a, b): (usize, usize) = parse_tuple2();
        let greater = a.max(b);
        less_vertex_counter[greater - 1] += 1;
    }
    // println!("{:?}", less_vertex_counter);
    println!(
        "{}",
        less_vertex_counter.into_iter().filter(|&i| i == 1).count()
    );
}
