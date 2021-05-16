use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (n, b, k): (usize, usize, usize) = parse_tuple3();
    let c: Vec<usize> = parse_values(k);
    let left_base = 10usize.pow((n - 1) as u32);
    let left = left_base + b - (left_base % b);
    let right_base = 10usize.pow(n as u32);
    let right = right_base - (right_base % b);
    let ans = (left..=right)
        .step_by(b)
        .filter(|&i| all_digit_included(i, &c))
        .count();
    println!("{}", ans);
}

fn all_digit_included(i: usize, numbers: &[usize]) -> bool {
    let mut i = i;
    while i != 0 {
        if numbers.contains(&(i % 10)) {
            i /= 10;
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::all_digit_included;

    #[test]
    fn test() {
        let numbers = [1, 4, 9];
        assert!(!all_digit_included(987, &numbers));
        //assert!(all_digit_included(994, &numbers));
    }
}
