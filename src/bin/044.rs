use competitive_tools_rust::io::{parse_tuple2, parse_tuple3, parse_values};

fn main() {
    let (n, query): (usize, usize) = parse_tuple2();
    let mut a_sequence: Vec<usize> = parse_values(n);
    let mut shift_count: usize = 0;
    for _ in 0..query {
        let (t, x, y): (usize, usize, usize) = parse_tuple3();
        // dbg!(&a_sequence);
        match t {
            1 => {
                let shifted_x = (n + x - shift_count - 1) % n;
                let shifted_y = (n + y - shift_count - 1) % n;
                // dbg!(shifted_x, shifted_y);
                a_sequence.swap(shifted_x, shifted_y);
            }
            2 => {
                shift_count += 1;
            }
            3 => {
                let shifted_x = (n + x - shift_count - 1) % n;
                // dbg!(shifted_x);
                println!("{}", a_sequence[shifted_x]);
            }
            _ => {}
        }
    }
}
