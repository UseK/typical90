use competitive_tools_rust::bitset::Bitset;
use competitive_tools_rust::io::{parse_tuple2, parse_values};
use itertools::Itertools;

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let p: Vec<Vec<usize>> = (0..h).map(|_| parse_values::<usize>(w)).collect();
    let mut ans: usize = 0;
    for bit_pattern in 0..2usize.pow(h as u32) {
        let true_bit_length = bit_pattern.to_bit_vec(h).iter().filter(|&&bit| bit).count();
        let mut counter: Vec<usize> = vec![0; h * w + 1];
        // println!("{:?}", bit_pattern.to_bit_vec(h));
        for j in 0..w {
            let vertical_nums: Vec<usize> = bit_pattern
                .to_bit_vec(h)
                .iter()
                .enumerate()
                .filter_map(|(i, bit)| if *bit { Some(p[i][j]) } else { None })
                .dedup()
                .collect();
            if vertical_nums.len() == 1 {
                // println!("{:?}", vertical_nums);
                counter[vertical_nums[0]] += true_bit_length;
                // println!("{:?}", counter);
            } else {
                // println!("{:?}", vertical_nums);
            }
        }
        ans = ans.max(counter.into_iter().max().unwrap());
    }
    println!("{}", ans);
}
