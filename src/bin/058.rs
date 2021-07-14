use competitive_tools_rust::io::parse_tuple2;

const LAW: usize = 10usize.pow(5);

fn main() {
    let (mut n, k): (usize, usize) = parse_tuple2();
    let mut log: Vec<usize> = vec![];
    let mut already = vec![false; LAW];
    for _ in 0..k {
        if already[n] {
            let n_pos = log.iter().position(|&x| x == n).unwrap();
            let log_looped = &log[n_pos..];
            let ans_ind = (k - log.len()) % log_looped.len();
            println!("{}", log_looped[ans_ind]);
            return;
        }
        already[n] = true;
        log.push(n);
        n = operate(n);
    }
    println!("{}", n);
}

fn operate(x: usize) -> usize {
    let mut digit_sum = 0;
    let mut current = x;
    while current > 0 {
        digit_sum += current % 10;
        current /= 10;
    }
    (x + digit_sum) % LAW
}
