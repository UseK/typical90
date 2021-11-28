use competitive_tools_rust::io::parse_tuple2;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let ab: Vec<(usize, usize)> = (0..n).map(|_| parse_tuple2::<usize, usize>()).collect();
    // println!("{:?}", ab);
    let mut scores = ab.iter().fold(vec![], |mut acc, ab_item| {
        acc.push(ab_item.1);
        acc.push(ab_item.0 - ab_item.1);
        acc
    });
    // println!("{:?}", scores);
    scores.sort_unstable();
    scores.reverse();
    // println!("{:?}", scores);
    println!("{:?}", &scores[0..k].iter().sum::<usize>());
}
