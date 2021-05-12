use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let s: Vec<char> = parse_line::<String>().chars().collect();

    let most_left_indices = gen_most_left_indices(n, &s);

    let mut current_ind = 0;
    let ans: String = (1..=k)
        .rev()
        .map(|current_k| {
            d!(current_k);
            let ans_char = ('a'..='z')
                .find(|&c| n - most_left_indices[c as usize - 97][current_ind] > current_k)
                .unwrap();
            d!(current_k, ans_char, current_ind);
            current_ind = most_left_indices[ans_char as usize - 97][current_ind] + 1;
            d!(current_k, ans_char, current_ind);
            ans_char
        })
        .collect();
    println!("{}", ans);
}

fn gen_most_left_indices(n: usize, s: &[char]) -> Vec<Vec<usize>> {
    let mut most_left_indices: Vec<Vec<usize>> = vec![vec![n; n]; 26];
    for c in 'a'..='z' {
        let most_left_ind = &mut most_left_indices[c as usize - 97];
        let mut current_ind = n;
        for s_ind in (0..n).rev() {
            if s[s_ind] == c {
                current_ind = s_ind
            }
            most_left_ind[s_ind] = current_ind;
        }
    }
    most_left_indices
}
