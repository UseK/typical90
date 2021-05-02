use competitive_tools_rust::bitset::Bitset as _;
//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let n: u32 = parse_line();
    for i in 1..2_usize.pow(n - 1) {
        //d!(i);
        let bits = i.to_bit_vec(n as usize);
        //d!(bits);
        if is_correct(&bits) {
            let parentheses: String = bits
                .into_iter()
                .map(|bit| if bit { '(' } else { ')' })
                .collect();
            println!("{}", parentheses);
        }
    }
}

fn is_correct(bits: &[bool]) -> bool {
    let mut true_count = 0;
    for &bit in bits {
        if bit {
            true_count += 1;
        } else if true_count == 0 {
            return false;
        } else {
            true_count -= 1;
        }
    }
    true_count == 0
}
