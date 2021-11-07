// use competitive_tools_rust::d;
use competitive_tools_rust::io::parse_line;

fn main() {
    let n: usize = parse_line();
    let s: String = parse_line();
    let rle = run_length_encoding(&s);
    // d!(rle);
    let ans = rle
        .iter()
        .fold(n * (n + 1) / 2, |sum, (_c, i)| sum - (i * (i + 1) / 2));
    println!("{}", ans);
}

fn run_length_encoding(s: &String) -> Vec<(char, usize)> {
    let s_chars: Vec<char> = s.chars().collect();
    let mut count = 1;
    let mut acc = vec![];
    for i in 1..s_chars.len() {
        if s_chars[i] == s_chars[i - 1] {
            count += 1;
        } else {
            acc.push((s_chars[i - 1], count));
            count = 1;
        }
    }
    acc.push((*s_chars.last().unwrap(), count));
    acc
}
