use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
use pathfinding::matrix::directions::N;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::vec;
use std::{
    any,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, Sub},
    path::Path,
};

fn applyV2(map: &mut Vec<Vec<usize>>, road: Vec<PosV2>) {
    for PosV2 { x, y, dir: _ } in road {
        map[x as usize][y as usize] = 0
    }
}

fn print_board(board: &Vec<Vec<usize>>) {
    for chars in board {
        for ch in chars {
            print!("{ch}");
        }
        println!()
    }
}

#[derive(Debug)]
enum Part {
    One,
    Two,
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    solve(path, Part::One)
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    solve(path, Part::Two)
}

fn solve<P: AsRef<Path>>(path: P, part: Part) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut map = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|ch| ch.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let res = match part {
        Part::One => astarv2(&mut map)?,
        Part::Two => astarv2_part2(&mut map)?,
    };

    println!("day17 part {:?}: res: {res}", part);

    Ok(())
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }

    fn velocity(&self) -> (i32, i32) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PosV2 {
    x: i32,
    y: i32,

    dir: Option<(usize, Dir)>,
}

impl PosV2 {
    fn successors(&self, board: &Vec<Vec<usize>>) -> Vec<(Self, u32)> {
        match self.dir {
            Some((steps, dir)) => {
                let possibilities = vec![
                    ((1, 0), Dir::Down),
                    ((-1, 0), Dir::Up),
                    ((0, 1), Dir::Right),
                    ((0, -1), Dir::Left),
                ];

                let mut successors = vec![];

                for ((dx, dy), new_dir) in possibilities {
                    let x = (self.x + dx) as usize;
                    let y = (self.y + dy) as usize;

                    if let Some(row) = board.get(x) {
                        if let Some(v) = row.get(y) {
                            if new_dir == dir && steps == 3 {
                                continue;
                            }
                            if new_dir == dir.opposite() {
                                continue;
                            }
                            let d = if new_dir == dir {
                                (steps + 1, dir)
                            } else {
                                (1, new_dir)
                            };

                            successors.push((
                                Self {
                                    x: x as i32,
                                    y: y as i32,
                                    dir: Some(d),
                                },
                                *v as u32,
                            ));
                        }
                    }
                }
                successors
            }
            None => {
                // starting point
                vec![
                    (
                        Self {
                            x: 0,
                            y: 1,
                            dir: Some((1, Dir::Right)),
                        },
                        board[0][1] as u32,
                    ),
                    (
                        Self {
                            x: 1,
                            y: 0,
                            dir: Some((1, Dir::Down)),
                        },
                        board[1][0] as u32,
                    ),
                ]
            }
        }
    }

    fn successors_part2(&self, board: &Vec<Vec<usize>>) -> Vec<(Self, u32)> {
        match self.dir {
            Some((steps, dir)) => {
                let mut successors = vec![];
                if steps < 4 {
                    let (dx, dy) = dir.velocity();
                    let x = (self.x + dx) as usize;
                    let y = (self.y + dy) as usize;

                    if let Some(row) = board.get(x) {
                        if let Some(v) = row.get(y) {
                            successors.push((
                                Self {
                                    x: x as i32,
                                    y: y as i32,
                                    dir: Some((steps + 1, dir)),
                                },
                                *v as u32,
                            ));
                        }
                    }
                } else {
                    let possibilities = vec![
                        ((1, 0), Dir::Down),
                        ((-1, 0), Dir::Up),
                        ((0, 1), Dir::Right),
                        ((0, -1), Dir::Left),
                    ];

                    for ((dx, dy), new_dir) in possibilities {
                        let x = (self.x + dx) as usize;
                        let y = (self.y + dy) as usize;

                        if let Some(row) = board.get(x) {
                            if let Some(v) = row.get(y) {
                                if new_dir == dir && steps == 10 {
                                    continue;
                                }
                                if new_dir == dir.opposite() {
                                    continue;
                                }
                                let d = if new_dir == dir {
                                    (steps + 1, dir)
                                } else {
                                    (1, new_dir)
                                };

                                successors.push((
                                    Self {
                                        x: x as i32,
                                        y: y as i32,
                                        dir: Some(d),
                                    },
                                    *v as u32,
                                ));
                            }
                        }
                    }
                }

                successors
            }
            None => {
                // starting point
                vec![
                    (
                        Self {
                            x: 0,
                            y: 1,
                            dir: Some((1, Dir::Right)),
                        },
                        board[0][1] as u32,
                    ),
                    (
                        Self {
                            x: 1,
                            y: 0,
                            dir: Some((1, Dir::Down)),
                        },
                        board[1][0] as u32,
                    ),
                ]
            }
        }
    }
}

fn heuristicV2(node: &PosV2, goal: &PosV2) -> u32 {
    // Euclidean distance heuristic
    ((node.x as isize - goal.x as isize).abs() + (node.y as isize - goal.y as isize).abs()) as u32
}

fn astarv2(board: &mut Vec<Vec<usize>>) -> anyhow::Result<u32> {
    let start = PosV2 {
        x: 0,
        y: 0,
        dir: None,
    };

    let goal = PosV2 {
        x: (board.len() - 1) as i32,
        y: (board[0].len() - 1) as i32,
        dir: None,
    };

    let res = pathfinding::prelude::astar(
        &start,
        |p| p.successors(board),
        |p| heuristicV2(p, &goal),
        |p| p.x == (board.len() - 1) as i32 && p.y == (board[0].len() - 1) as i32,
    )
    .context("path not find")?;

    Ok(res.1)
}

fn astarv2_part2(board: &mut Vec<Vec<usize>>) -> anyhow::Result<u32> {
    let start = PosV2 {
        x: 0,
        y: 0,
        dir: None,
    };

    let goal = PosV2 {
        x: (board.len() - 1) as i32,
        y: (board[0].len() - 1) as i32,
        dir: None,
    };

    let res = pathfinding::prelude::astar(
        &start,
        |p| p.successors_part2(board),
        |p| heuristicV2(p, &goal),
        |p| p.x == (board.len() - 1) as i32 && p.y == (board[0].len() - 1) as i32,
    )
    .context("path not find")?;

    Ok(res.1)
}
