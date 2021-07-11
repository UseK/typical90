use competitive_tools_rust::io::{parse_line, parse_values};

fn main() {
    let n: usize = parse_line();
    let a: Vec<usize> = parse_values(n);
    let b: Vec<usize> = parse_values(n);
    let c: Vec<usize> = parse_values(n);

    let a_group = mod_46_grouped(&a);
    let b_group = mod_46_grouped(&b);
    let c_group = mod_46_grouped(&c);
    // println!("{:?}", a_group);
    // println!("{:?}", b_group);
    // println!("{:?}", c_group);

    let mut count = 0;
    for i in 0..46 {
        for j in 0..46 {
            if i + j == 0 {
                continue;
            };
            let remain = if i + j <= 46 {
                46 - (i + j)
            } else {
                92 - (i + j)
            };
            dbg!(i, j, remain);
            count += a_group[i] * b_group[j] * c_group[remain];
        }
    }
    println!("{}", count);
}

fn mod_46_grouped(x: &Vec<usize>) -> Vec<usize> {
    let mut grouped: Vec<usize> = vec![0; 46];
    for i in x {
        grouped[i % 46] += 1;
    }
    grouped
}
