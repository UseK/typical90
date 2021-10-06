//use competitive_tools_rust::d;
use competitive_tools_rust::io::*;
use competitive_tools_rust::segment_tree::SegmentTree;

fn main() {
    let (n, k): (usize, usize) = parse_tuple2();
    let s: String = parse_line();
    let cs: Vec<char> = s.chars().collect();
    let mut seg: SegmentTree<(char, usize)> = SegmentTree::new(n, ('z', usize::MAX));
    (0..n).for_each(|i| {
        seg.update(i, (cs[i], i));
    });
    // println!("{:?}", seg.tree);

    let mut left = 0;
    let answer: String = (n - k + 1..=n)
        .map(|right| {
            let ans = seg.query(left, right);
            // println!("{:?}", ans);
            left = ans.1 + 1;
            ans.0
        })
        .collect();
    println!("{}", answer);
}
