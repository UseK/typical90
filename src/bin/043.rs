// use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_tuple2};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let (start_r, start_c): (usize, usize) = parse_tuple2();
    let (target_r, target_c): (usize, usize) = parse_tuple2();
    let mut maze: Vec<Vec<bool>> = (0..h)
        .map(|_| parse_line::<String>().chars().map(|s| s == '.').collect())
        .collect();
    surround_wall(&mut maze);
    let maze = maze;
    let mut counter: Vec<Vec<usize>> = vec![vec![usize::MAX; w + 2]; h + 2];
    let mut togo_queue: BinaryHeap<Reverse<ToGo>> = BinaryHeap::new();
    togo_queue.append(
        &mut ToGo::new_all_directions(start_r, start_c, 0)
            .into_iter()
            .map(Reverse)
            .collect(),
    );
    loop {
        let Reverse(togo) = togo_queue.pop().unwrap();
        // println!("{:?}", togo);
        let (next_r, next_c) = match togo.direction {
            Direction::Up => (togo.r - 1, togo.c),
            Direction::Down => (togo.r + 1, togo.c),
            Direction::Left => (togo.r, togo.c - 1),
            Direction::Right => (togo.r, togo.c + 1),
        };
        if (next_r, next_c) == (target_r, target_c) {
            println!("{}", togo.count);
            return;
        }
        if counter[next_r][next_c] < togo.count {
            continue;
        } else {
            counter[next_r][next_c] = togo.count;
        }
        if maze[next_r][next_c] {
            togo_queue.push(Reverse(ToGo::new(
                next_r,
                next_c,
                togo.direction,
                togo.count,
            )));
            let next_togo_list = match togo.direction {
                Direction::Up => ToGo::new_horizontal_directions(next_r, next_c, togo.count + 1),
                Direction::Down => ToGo::new_horizontal_directions(next_r, next_c, togo.count + 1),
                Direction::Left => ToGo::new_vertical_directions(next_r, next_c, togo.count + 1),
                Direction::Right => ToGo::new_vertical_directions(next_r, next_c, togo.count + 1),
            };
            togo_queue.append(&mut next_togo_list.into_iter().map(Reverse).collect());
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ToGo {
    count: usize,
    r: usize,
    c: usize,
    direction: Direction,
}

impl ToGo {
    fn new(r: usize, c: usize, direction: Direction, count: usize) -> Self {
        ToGo {
            count,
            r,
            c,
            direction,
        }
    }

    fn new_vertical_directions(r: usize, c: usize, count: usize) -> Vec<ToGo> {
        vec![
            Self::new(r, c, Direction::Up, count),
            Self::new(r, c, Direction::Down, count),
        ]
    }

    fn new_horizontal_directions(r: usize, c: usize, count: usize) -> Vec<ToGo> {
        vec![
            Self::new(r, c, Direction::Left, count),
            Self::new(r, c, Direction::Right, count),
        ]
    }

    fn new_all_directions(r: usize, c: usize, count: usize) -> Vec<ToGo> {
        vec![
            Self::new(r, c, Direction::Up, count),
            Self::new(r, c, Direction::Down, count),
            Self::new(r, c, Direction::Left, count),
            Self::new(r, c, Direction::Right, count),
        ]
    }
}

fn surround_wall(maze: &mut Vec<Vec<bool>>) {
    maze.iter_mut().for_each(|line| {
        line.insert(0, false);
        line.push(false);
    });
    let n_line = maze[0].len();
    maze.insert(0, vec![false; n_line]);
    maze.push(vec![false; n_line]);
}
