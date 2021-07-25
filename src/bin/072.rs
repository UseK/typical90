use competitive_tools_rust::d;
use competitive_tools_rust::graph::{dijkstra, Edge};
use competitive_tools_rust::io::parse_tuple2;
use competitive_tools_rust::maze::{parse_maze, surround_wall};

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let mut maze = parse_maze(h, '#');
    surround_wall(&mut maze);
    // maze.iter().for_each(|item| println!("{:?}", item));
    let mut edges_list: Vec<Vec<Edge>> = vec![vec![]; h * w];
    for i in 1..=h {
        for j in 1..=w {
            if !maze[i][j] {
                continue;
            };
            for &(diff_i, diff_j) in &[(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
                if maze[diff_i][diff_j] {
                    edges_list[(i - 1) * h + (j - 1)].push(Edge {
                        to: (diff_i - 1) * h + (diff_j - 1),
                        cost: 1,
                    })
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            let result = dijkstra(i * h + j, h * w, &edges_list);
            d!(i, j);
            d!(result);
        }
    }
}
