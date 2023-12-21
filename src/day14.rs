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

    let mut board = BufReader::new(file)
        .lines()
        .map(Result::ok)
        .map(Option::unwrap)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    roll(&mut board, Direction::North);

    print_board(&board);

    let mut res = 0;

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                res += board.len() - (i);
            }
        }
    }

    println!("result day14a : {res}",);
    Ok(())
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut board = BufReader::new(file)
        .lines()
        .map(Result::ok)
        .map(Option::unwrap)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    // 10 -> north, (w,s,e,n)*2

    let mut cache = HashMap::new();
    for i in 0..1_000_000_000 {
        println!("LOOP {i}");
        roll(&mut board, Direction::North); // 1 5 9
        roll(&mut board, Direction::West); // 2 6 10
        roll(&mut board, Direction::South); // 3 7
        roll(&mut board, Direction::East); // 4 8

        if let Some(x) = cache.get(&board) {
            let cycle = i + 1 - x;
            let remaining = (1_000_000_000 - i - 1) % cycle;

            for _ in 0..remaining {
                roll(&mut board, Direction::North); // 1 5 9
                roll(&mut board, Direction::West); // 2 6 10
                roll(&mut board, Direction::South); // 3 7
                roll(&mut board, Direction::East); // 4 8
            }
            break;
        }

        cache.insert(board.clone(), i + 1);
    }

    let mut res = 0;

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 'O' {
                res += board.len() - (i);
            }
        }
    }

    println!("result day14b : {res}",);
    Ok(())
}

enum Direction {
    North,
    East,
    South,
    West,
}

// impl Direction {
//     fn get_current_and_next(&self) -> ((usize, usize), (usize, usize)) {

//     }
// }

fn roll(board: &mut Vec<Vec<char>>, dir: Direction) {
    match dir {
        Direction::North => loop {
            let mut moved = false;
            for i in 1..board.len() {
                for j in 0..board[0].len() {
                    if board[i][j] == 'O' {
                        let above = board[i - 1][j];
                        if above != 'O' && above != '#' {
                            moved = true;
                            board[i - 1][j] = 'O';
                            board[i][j] = '.';
                        }
                    }
                }
            }

            if !moved {
                break;
            }
        },
        Direction::East => loop {
            let mut moved = false;
            for i in 0..board.len() {
                for j in 0..board[0].len() - 1 {
                    if board[i][j] == 'O' {
                        let after = board[i][j + 1];
                        if after != 'O' && after != '#' {
                            moved = true;
                            board[i][j + 1] = 'O';
                            board[i][j] = '.';
                        }
                    }
                }
            }

            if !moved {
                break;
            }
        },
        Direction::South => loop {
            let mut moved = false;
            for i in (0..board.len() - 1).rev() {
                for j in 0..board[0].len() {
                    if board[i][j] == 'O' {
                        let below = board[i + 1][j];
                        if below != 'O' && below != '#' {
                            moved = true;
                            board[i + 1][j] = 'O';
                            board[i][j] = '.';
                        }
                    }
                }
            }

            if !moved {
                break;
            }
        },
        Direction::West => loop {
            let mut moved = false;
            for i in 0..board.len() {
                for j in (1..board[0].len()).rev() {
                    if board[i][j] == 'O' {
                        let before = board[i][j - 1];
                        if before != 'O' && before != '#' {
                            moved = true;
                            board[i][j - 1] = 'O';
                            board[i][j] = '.';
                        }
                    }
                }
            }

            if !moved {
                break;
            }
        },
    }
}
