use competitive_tools_rust::bitset::Bitset as _;
//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let n: u32 = parse_line();
    for i in 1..2_usize.pow(n - 1) {
        let bits: Vec<bool> = i.to_bit_vec(n as usize).into_iter().rev().collect();
        if is_correct(&bits) {
            //d!(i);
            //d!(bits);
            let parentheses: String = bits
                .into_iter()
                .map(|bit| if !bit { '(' } else { ')' })
                .collect();
            println!("{}", parentheses);
        }
    }
}

fn is_correct(bits: &[bool]) -> bool {
    let mut false_count = 0;
    for &bit in bits {
        if !bit {
            false_count += 1;
        } else if false_count == 0 {
            return false;
        } else {
            false_count -= 1;
        }
    }
    false_count == 0
}
