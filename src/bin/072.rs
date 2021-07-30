// use competitive_tools_rust::d;
use competitive_tools_rust::io::parse_tuple2;
use competitive_tools_rust::maze::{parse_maze, surround_wall};

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let mut maze = parse_maze(h, '#');
    surround_wall(&mut maze);
    let mut max_count = 0;
    for i in 1..=h {
        for j in 1..=w {
            if !maze[i][j] {
                continue;
            };
            let count = search_max_route(maze.clone(), i, j, i, j, 0);
            max_count = max_count.max(count);
            // d!(i, j, count, max_count);
        }
    }
    if max_count == 0 {
        println!("-1");
    } else {
        println!("{}", max_count);
    }
}

fn search_max_route(
    mut maze: Vec<Vec<bool>>,
    start_i: usize,
    start_j: usize,
    i: usize,
    j: usize,
    count: usize,
) -> usize {
    if count >= 4 && i == start_i && j == start_j {
        return count;
    }
    if !maze[i][j] {
        0
    } else {
        maze[i][j] = false;
        let incremented = count + 1;
        vec![
            search_max_route(maze.clone(), start_i, start_j, i - 1, j, incremented),
            search_max_route(maze.clone(), start_i, start_j, i + 1, j, incremented),
            search_max_route(maze.clone(), start_i, start_j, i, j - 1, incremented),
            search_max_route(maze.clone(), start_i, start_j, i, j + 1, incremented),
        ]
        .into_iter()
        .max()
        .unwrap()
    }
}
