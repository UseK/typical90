use competitive_tools_rust::bitset::Bitset;
use competitive_tools_rust::io::*;

fn main() {
    let n: usize = parse_line();
    (0..2usize.pow(n as u32)).for_each(|i| {
        let bits: Vec<bool> = i.to_bit_vec(n).into_iter().rev().collect();
        // d!(i, bits, is_same_count(&bits), is_all_close(&bits));
        if is_valid(&bits) {
            let s: String = bits
                .into_iter()
                .map(|bit| if bit { ')' } else { '(' })
                .collect();
            println!("{}", s);
        }
    })
}

fn is_valid(bits: &[bool]) -> bool {
    let mut count_open = 0;
    bits.iter().all(|&bit| {
        if bit {
            count_open -= 1;
        } else {
            count_open += 1;
        }
        count_open >= 0
    }) && count_open == 0
}
