use competitive_tools_rust::bitset::Bitset;
use competitive_tools_rust::io::parse_tuple2;

const LAW: usize = 10usize.pow(9) + 7;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();

    let corner = match (n, k) {
        (1, k) => Some(k),
        (_, 1) => Some(0),
        (2, 2) => Some(2),
        (_, 2) => Some(0),
        (_, _) => None,
    };
    if let Some(x) = corner {
        println!("{}", x);
        return;
    }

    println!("{}", k * (k - 1) % LAW * (k - 2).mod_pow(n - 2, LAW) % LAW);
}

trait ModPow {
    /// Returns Modular Exponentiation  with
    fn mod_pow(self, exp: Self, m: Self) -> Self;
}

impl ModPow for usize {
    fn mod_pow(self, exp: Self, m: Self) -> Self {
        let mut acc = 1;
        let mut current_mod_pow = self % m;
        for i in 0..exp {
            if 2usize.pow(i as u32) > exp {
                break;
            }
            if exp.is_bit_on(i) {
                acc = (acc * current_mod_pow) % m;
            }
            current_mod_pow = (current_mod_pow * current_mod_pow) % m;
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use crate::ModPow;
    const LAW: usize = 10usize.pow(9) + 7;

    fn naive_modular_exponentiation(b: usize, e: usize, m: usize) -> usize {
        let mut exp = 1;
        for _ in 0..e {
            exp = (exp * b) % m;
        }
        exp
    }

    #[test]
    fn test_mod_pow_in_small_case() {
        let b = 2;
        let exp = 10;
        assert_eq!(b.mod_pow(exp, 5), 4);
    }

    #[test]
    fn test_mod_pow_in_small_case2() {
        let b = 2;
        let exp = 4;
        assert_eq!(b.mod_pow(exp, 5), 1);
    }

    #[test]
    fn test_mod_pow_in_big_case() {
        let b = 2019;
        let exp = 615;
        assert_eq!(b.mod_pow(exp, LAW), 492000830);
        assert_eq!(naive_modular_exponentiation(b, exp, LAW), 492000830);
    }

    #[test]
    fn test_mod_pow_in_all_pattern() {
        for b in 0..100 {
            for exp in 0..100 {
                for m in 2..100 {
                    assert_eq!(b.mod_pow(exp, m), naive_modular_exponentiation(b, exp, m));
                }
            }
        }
    }
}
