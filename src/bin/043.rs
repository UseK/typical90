use competitive_tools_rust::d;
use competitive_tools_rust::io::{parse_line, parse_tuple2};

fn main() {
    let (h, w): (usize, usize) = parse_tuple2();
    let (start_r, start_c): (usize, usize) = parse_tuple2();
    let (target_r, target_c): (usize, usize) = parse_tuple2();
    let mut maze: Vec<Vec<bool>> = (0..h)
        .map(|_| parse_line::<String>().chars().map(|s| s == '.').collect())
        .collect();
    surround_wall(&mut maze);
    let maze = maze;
    maze.iter().for_each(|item| println!("{:?}", item));

    let mut counter: Vec<Vec<usize>> = vec![vec![usize::MAX; w + 2]; h + 2];
    let mut togo_stock: Vec<ToGo> = ToGo::new_all_directions(start_r, start_c, 0);
    loop {
        let togo = togo_stock.pop().unwrap();
        counter[togo.r][togo.c] = counter[togo.r][togo.c].min(togo.count);

        let (next_r, next_c) = match togo.direction {
            Direction::Up => (togo.r - 1, togo.c),
            Direction::Down => (togo.r + 1, togo.c),
            Direction::Left => (togo.r, togo.c - 1),
            Direction::Right => (togo.r, togo.c + 1),
        };
        if (next_r, next_c) == (target_r, target_c) {
            d!(next_r, next_c, togo);
            println!("{}", togo.count);
            return;
        }
        if maze[next_r][next_c] {
            let mut next_togo_list = match togo.direction {
                Direction::Up => ToGo::new_horizontal_directions(next_r, next_c, togo.count + 1),
                Direction::Down => ToGo::new_horizontal_directions(next_r, next_c, togo.count + 1),
                Direction::Left => ToGo::new_vertical_directions(next_r, next_c, togo.count + 1),
                Direction::Right => ToGo::new_vertical_directions(next_r, next_c, togo.count + 1),
            };
            togo_stock.append(&mut next_togo_list);
            togo_stock.push(ToGo::new(next_r, next_c, togo.direction, togo.count));

            println!("{:?}", togo);
            println!("----------");
            togo_stock.iter().for_each(|item| println!("{:?}", item));
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct ToGo {
    r: usize,
    c: usize,
    direction: Direction,
    count: usize,
}

impl ToGo {
    fn new(r: usize, c: usize, direction: Direction, count: usize) -> Self {
        ToGo {
            r,
            c,
            direction,
            count,
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
