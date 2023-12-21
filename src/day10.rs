use itertools::Itertools;
use std::collections::VecDeque;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn print_board(board: &Vec<Vec<char>>) {
    for chars in board {
        println!("{:?}", chars);
    }
}
fn print_board_string(board: &Vec<Vec<String>>) {
    for chars in board {
        println!("{:?}", chars);
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let board: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect_vec())
        .collect();

    print_board(&board);
    println!();
    let mut result_board = board.clone();

    let mut starting_point = (0, 0);
    'outer: for (i, chars) in board.iter().enumerate() {
        for (j, value) in chars.iter().enumerate() {
            if *value == 'S' {
                starting_point = (i, j);
                break 'outer;
            }
        }
    }
    result_board[starting_point.0][starting_point.1] = '#';

    let mut additional_board = result_board
        .clone()
        .into_iter()
        .map(|chs| chs.into_iter().map(|ch| ch.to_string()).collect_vec())
        .collect_vec();

    let result_distance = walk_board(
        &board,
        &mut additional_board,
        &mut HashSet::new(),
        vec![(starting_point, Direction::Up, None)].into(),
        0,
    );

    let mut edges = vec![];
    walk_board_get_edges(
        &board,
        &mut HashSet::new(),
        &mut edges,
        vec![DequePoint {
            point: starting_point,
            prev_point: (0, 0),
            dir: Direction::None,
            prev_dir: Direction::None,
            starting_point,
            prev: None,
        }]
        .into(),
    );

    dbg!(&edges);
    let mut points = 0;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if is_inside(&edges, (i as f32, j as f32)) {
                println!("point inside loop: {:?}, {}", (i, j), board[i][j]);
                points += 1;
            }
        }
    }

    println!("points: {points}");

    println!();
    print_board_string(&additional_board);

    println!("result day10 a: {}", result_distance - 1);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}

fn walk_board(
    board: &Vec<Vec<char>>,
    res: &mut Vec<Vec<String>>,
    visited: &mut HashSet<(usize, usize)>,
    next_to_visit: VecDeque<((usize, usize), Direction, Option<char>)>,

    dist: usize,
) -> usize {
    let mut ntv = VecDeque::new();
    for ((x, y), dir, prev) in next_to_visit {
        match prev {
            Some(prev) => {
                if let Some(chars) = board.get(x) {
                    if let Some(curr) = chars.get(y) {
                        if curr == &'.' {
                            continue;
                        }
                        match dir {
                            Direction::Up => {
                                if *curr != 'F' && *curr != '7' && *curr != '|' {
                                    continue;
                                }
                                if prev != '|' && prev != 'L' && prev != 'J' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Down => {
                                if *curr != 'L' && *curr != 'J' && *curr != '|' {
                                    continue;
                                }
                                if prev != '|' && prev != 'F' && prev != '7' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Right => {
                                if *curr != '-' && *curr != '7' && *curr != 'J' {
                                    continue;
                                }
                                if prev != '-' && prev != 'L' && prev != 'F' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Left => {
                                if *curr != '-' && *curr != 'L' && *curr != 'F' {
                                    continue;
                                }
                                if prev != '-' && prev != '7' && prev != 'J' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::None => {}
                        }
                        match visited.get(&(x, y)) {
                            Some(_) => {
                                continue;
                            }
                            None => {
                                visited.insert((x, y));
                            }
                        }
                        res[x][y] = dist.to_string();

                        ntv.push_front(((x + 1, y), Direction::Down, Some(*curr)));

                        if let Some(n) = x.checked_sub(1) {
                            ntv.push_front(((n, y), Direction::Up, Some(*curr)));
                        }
                        ntv.push_front(((x, y + 1), Direction::Right, Some(*curr)));

                        if let Some(n) = y.checked_sub(1) {
                            ntv.push_front(((x, n), Direction::Left, Some(*curr)));
                        }
                    }
                }
            }

            None => {
                ntv.push_front(((x + 1, y), Direction::Down, Some('S')));

                if let Some(n) = x.checked_sub(1) {
                    ntv.push_front(((n, y), Direction::Up, Some('S')));
                }
                ntv.push_front(((x, y + 1), Direction::Right, Some('S')));

                if let Some(n) = y.checked_sub(1) {
                    ntv.push_front(((x, n), Direction::Left, Some('S')));
                }
            }
        }
    }
    if ntv.is_empty() {
        return dist;
    }
    walk_board(board, res, visited, ntv, dist + 1)
}

#[derive(Debug)]
struct DequePoint {
    point: (usize, usize),
    prev_point: (usize, usize),
    dir: Direction,
    prev_dir: Direction,
    starting_point: (usize, usize),
    prev: Option<char>,
}

fn walk_board_get_edges(
    board: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    edges: &mut Vec<Edge>,
    next_to_visit: VecDeque<DequePoint>,
) {
    let mut ntv = VecDeque::new();
    for point in next_to_visit {
        let (x, y) = point.point;

        match point.prev {
            Some(prev) => {
                if let Some(chars) = board.get(x) {
                    if let Some(curr) = chars.get(y) {
                        if curr == &'.' {
                            continue;
                        }
                        match point.dir {
                            Direction::Up => {
                                if *curr != 'F' && *curr != '7' && *curr != '|' {
                                    continue;
                                }
                                if prev != '|' && prev != 'L' && prev != 'J' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Down => {
                                if *curr != 'L' && *curr != 'J' && *curr != '|' {
                                    continue;
                                }
                                if prev != '|' && prev != 'F' && prev != '7' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Right => {
                                if *curr != '-' && *curr != '7' && *curr != 'J' {
                                    continue;
                                }
                                if prev != '-' && prev != 'L' && prev != 'F' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::Left => {
                                if *curr != '-' && *curr != 'L' && *curr != 'F' {
                                    continue;
                                }
                                if prev != '-' && prev != '7' && prev != 'J' && prev != 'S' {
                                    continue;
                                }
                            }
                            Direction::None => {}
                        }
                        match visited.get(&(x, y)) {
                            Some(_) => {
                                continue;
                            }
                            None => {
                                visited.insert((x, y));
                            }
                        }
                        let mut prev_dir = point.dir;
                        let mut startin_point = point.starting_point;
                        if point.prev_dir != point.dir && point.prev_dir != Direction::None {
                            edges.push(Edge {
                                x1: point.starting_point.0 as f32,
                                x2: point.prev_point.0 as f32,
                                y1: point.starting_point.1 as f32,
                                y2: point.prev_point.1 as f32,
                            });
                            // pivot, save edge
                            println!("edge: {:?}, {:?}", point.starting_point, point.prev_point);
                            prev_dir = Direction::None;
                            startin_point = point.prev_point;
                        }

                        ntv.push_front(DequePoint {
                            point: (x + 1, y),
                            prev_point: (x, y),
                            dir: Direction::Down,
                            prev: Some(*curr),
                            prev_dir: prev_dir,
                            starting_point: startin_point,
                        });

                        if let Some(n) = x.checked_sub(1) {
                            ntv.push_front(DequePoint {
                                point: (n, y),
                                prev_point: (x, y),
                                dir: Direction::Up,
                                prev: Some(*curr),
                                prev_dir: prev_dir,
                                starting_point: startin_point,
                            });
                        }
                        ntv.push_front(DequePoint {
                            point: (x, y + 1),
                            prev_point: (x, y),
                            dir: Direction::Right,
                            prev: Some(*curr),
                            prev_dir: prev_dir,
                            starting_point: startin_point,
                        });

                        if let Some(n) = y.checked_sub(1) {
                            ntv.push_front(DequePoint {
                                point: (x, n),
                                prev_point: (x, y),
                                dir: Direction::Left,
                                prev: Some(*curr),
                                prev_dir: prev_dir,
                                starting_point: startin_point,
                            });
                        }
                    }
                }
            }

            None => {
                ntv.push_front(DequePoint {
                    point: (x + 1, y),
                    prev_point: (x, y),
                    dir: Direction::Down,
                    prev: Some('S'),
                    prev_dir: Direction::None,
                    starting_point: (x, y),
                });

                if let Some(n) = x.checked_sub(1) {
                    ntv.push_front(DequePoint {
                        point: (n, y),
                        prev_point: (x, y),
                        dir: Direction::Up,
                        prev: Some('S'),
                        prev_dir: Direction::None,
                        starting_point: (x, y),
                    });
                }
                ntv.push_front(DequePoint {
                    point: (x, y + 1),
                    prev_point: (x, y),
                    dir: Direction::Right,
                    prev: Some('S'),
                    prev_dir: Direction::None,
                    starting_point: (x, y),
                });

                if let Some(n) = y.checked_sub(1) {
                    ntv.push_front(DequePoint {
                        point: (x, n),
                        prev_point: (x, y),
                        dir: Direction::Left,
                        prev: Some('S'),
                        prev_dir: Direction::None,
                        starting_point: (x, y),
                    });
                }
            }
        }
    }
    if ntv.is_empty() {
        return;
    }

    walk_board_get_edges(board, visited, edges, ntv)
}

#[derive(Debug)]
struct Edge {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

fn is_inside(edges: &Vec<Edge>, point: (f32, f32)) -> bool {
    let mut count = 0;
    let (yp, xp) = (point.0, point.1);

    for edge in edges {
        let (x1, y1, x2, y2) = (edge.x1, edge.y1, edge.x2, edge.y2);
        if (yp < y1) != (yp < y2) && xp < x1 + ((yp - y1) / (y2 - y1) * (x2 - x1)) {
            count += 1
        }
    }
    return count % 2 == 1;
}

mod tests {
    use super::Edge;
    use crate::day10::is_inside;

    #[test]
    fn test_is_inside() {
        let edges = vec![
            Edge {
                x1: 0.,
                x2: 0.,
                y1: 0.,
                y2: 4.,
            },
            Edge {
                x1: 0.,
                x2: 3.,
                y1: 4.,
                y2: 4.,
            },
            Edge {
                x1: 3.,
                x2: 3.,
                y1: 4.,
                y2: 3.,
            },
            Edge {
                x1: 3.,
                x2: 1.,
                y1: 3.,
                y2: 3.,
            },
            Edge {
                x1: 1.,
                x2: 1.,
                y1: 3.,
                y2: 1.,
            },
            Edge {
                x1: 1.,
                x2: 3.,
                y1: 1.,
                y2: 1.,
            },
            Edge {
                x1: 3.,
                x2: 3.,
                y1: 1.,
                y2: 0.,
            },
            Edge {
                x1: 3.,
                x2: 0.,
                y1: 0.,
                y2: 0.,
            },
        ];
        assert_eq!(is_inside(&edges, (2., 2.)), false)
    }
}
