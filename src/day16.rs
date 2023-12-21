use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
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

fn print_board(board: &Vec<Vec<char>>) {
    for chars in board {
        println!("{:?}", chars);
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut map = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut energized = HashSet::new();
    walk(
        &mut map,
        &mut energized,
        &mut HashSet::new(),
        (0, 0),
        Dir::Right,
    );

    println!("result day16a: {}", energized.len());

    Ok(())
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

enum Mirror {
    Left,  // \
    Right, // /
}

impl Dir {
    fn velocity(&self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }

    fn adjust(self, mirror: Mirror) -> Self {
        match (self, mirror) {
            (Dir::Right, Mirror::Left) => Self::Down,
            (Dir::Right, Mirror::Right) => Self::Up,
            (Dir::Left, Mirror::Left) => Self::Up,
            (Dir::Left, Mirror::Right) => Self::Down,
            (Dir::Up, Mirror::Left) => Self::Left,
            (Dir::Up, Mirror::Right) => Self::Right,
            (Dir::Down, Mirror::Left) => Self::Right,
            (Dir::Down, Mirror::Right) => Self::Left,
        }
    }
}

impl Into<char> for Dir {
    fn into(self) -> char {
        match self {
            Dir::Right => '>',
            Dir::Left => '<',
            Dir::Up => 'V',
            Dir::Down => '^',
        }
    }
}

fn add_to_usize(point: (usize, usize), another: (isize, isize)) -> (isize, isize) {
    (point.0 as isize + another.0, point.1 as isize + another.1)
}

fn walk(
    map: &mut Vec<Vec<char>>,
    energized: &mut HashSet<(isize, isize)>,
    visited: &mut HashSet<((isize, isize), Dir)>,
    point: (isize, isize),
    mut dir: Dir,
) {
    if visited.contains(&(point, dir)) {
        return;
    }

    match (
        TryInto::<usize>::try_into(point.0),
        TryInto::<usize>::try_into(point.1),
    ) {
        (Ok(x), Ok(y)) => {
            if let Some(row) = map.get(x) {
                if let Some(ch) = row.get(y) {
                    visited.insert((point, dir));
                    energized.insert(point);
                    match ch {
                        '.' => map[x][y] = dir.into(),
                        '\\' => dir = dir.adjust(Mirror::Left),
                        '/' => dir = dir.adjust(Mirror::Right),
                        '|' => match dir {
                            Dir::Right | Dir::Left => {
                                walk(
                                    map,
                                    energized,
                                    &mut visited.clone(),
                                    add_to_usize((x, y), Dir::Up.velocity()),
                                    Dir::Up,
                                );
                                walk(
                                    map,
                                    energized,
                                    visited,
                                    add_to_usize((x, y), Dir::Down.velocity()),
                                    Dir::Down,
                                );

                                return;
                            }
                            _ => (),
                        },
                        '-' => match dir {
                            Dir::Up | Dir::Down => {
                                walk(
                                    map,
                                    energized,
                                    &mut visited.clone(),
                                    add_to_usize((x, y), Dir::Left.velocity()),
                                    Dir::Left,
                                );
                                walk(
                                    map,
                                    energized,
                                    visited,
                                    add_to_usize((x, y), Dir::Right.velocity()),
                                    Dir::Right,
                                );

                                return;
                            }
                            _ => (),
                        },
                        '>' | '<' | '^' | 'v' => map[x][y] = '2',
                        _ => (),
                    }

                    walk(
                        map,
                        energized,
                        visited,
                        add_to_usize((x, y), dir.velocity()),
                        dir,
                    )
                }
            }
        }
        _ => (),
    }
}
