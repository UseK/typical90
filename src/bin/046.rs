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
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    let adder = a_group[i] * b_group[j] * c_group[k];
                    count += adder;
                }
            }
        }
    }
    println!("{}", count);
}

fn mod_46_grouped(x: &[usize]) -> Vec<usize> {
    let mut grouped: Vec<usize> = vec![0; 46];
    for i in x {
        grouped[i % 46] += 1;
    }
    grouped
}
