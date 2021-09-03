// use competitive_tools_rust::d;
use competitive_tools_rust::io::*;

fn main() {
    let (w, n): (usize, usize) = parse_tuple2();
    let mut stack: Vec<Vec<bool>> = vec![vec![true; w], vec![false; w]];
    for _ in 0..n {
        let (l, r): (usize, usize) = parse_tuple2();
        for h in (1..stack.len()).rev() {
            let is_vacant = stack[h][l - 1..r].iter().all(|&x| !x);
            let can_put = stack[h - 1][l - 1..r].iter().any(|&x| x);
            if is_vacant && can_put {
                for i in l - 1..r {
                    stack[h][i] = true;
                }
                if h == stack.len() - 1 {
                    stack.push(vec![false; w]);
                }
                println!("{}", h);
                break;
            }
        }
        // show_maze(&stack);
    }
}

// fn show_maze(maze: &[Vec<bool>]) {
//     maze.iter().for_each(|line| {
//         let line_str: String = line.iter().map(|&b| if b { '#' } else { '.' }).collect();
//         println!("{}", line_str);
//     });
//     println!("{}", (0..maze[0].len()).map(|_| '-').collect::<String>());
// }
