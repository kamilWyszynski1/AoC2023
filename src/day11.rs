use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
use std::hash::Hash;
use std::{
    any,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, Sub},
    path::Path,
};

fn print_board_string(board: &Vec<Vec<String>>) {
    for chars in board {
        for ch in chars {
            print!("{ch}")
        }
        println!()
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut board = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|ch| ch.to_string())
                .collect_vec()
        })
        .collect_vec();

    let mut inxes = vec![];
    for (inx, lines) in board.iter().enumerate() {
        if lines.iter().all(|s| s == ".") {
            inxes.push(inx)
        }
    }

    for inx in inxes.into_iter().rev() {
        board.insert(
            inx,
            std::iter::repeat(String::from("."))
                .take(board[0].len())
                .collect_vec(),
        )
    }

    let mut cls = vec![];
    'cols: for i in 0..board[0].len() {
        for line in &board {
            if line[i] != "." {
                continue 'cols;
            }
        }
        cls.push(i);
    }

    for col in cls.into_iter().rev() {
        for line in board.iter_mut() {
            line.insert(col, String::from("."))
        }
    }

    print_board_string(&board);

    let mut galaxies = vec![];

    board.into_iter().enumerate().for_each(|(x, line)| {
        line.into_iter().enumerate().for_each(|(y, value)| {
            if value == "#".to_string() {
                galaxies.push((x as isize, y as isize));
            }
        })
    });

    // 5 -> (6,1) 9-> (11,5) = 11-6 + 5-1 = 9

    let res: isize = galaxies
        .iter()
        .permutations(2)
        .map(|v| (v[0].0, v[0].1, v[1].0, v[1].1))
        .map(|(x1, y1, x2, y2)| {
            let d = (x1 - x2).abs() + (y1 - y2).abs();
            d
        })
        .sum();

    println!("result day11:  {}", res / 2);

    Ok(())
}

const SPACE: isize = 100 - 1;

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut board = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|ch| ch.to_string())
                .collect_vec()
        })
        .collect_vec();

    let mut inxes = vec![];
    for (inx, lines) in board.iter().enumerate() {
        if lines.iter().all(|s| s == ".") {
            inxes.push(inx)
        }
    }

    let mut cls = vec![];
    'cols: for i in 0..board[0].len() {
        for line in &board {
            if line[i] != "." {
                continue 'cols;
            }
        }
        cls.push(i);
    }

    let mut galaxies = vec![];

    board.into_iter().enumerate().for_each(|(x, line)| {
        line.into_iter().enumerate().for_each(|(y, value)| {
            if value == "#".to_string() {
                galaxies.push((x as isize, y as isize));
            }
        })
    });

    dbg!(&galaxies);

    // 0, 2 -> 4, 5
    // 4, 5 -> 0, 2

    let pers: HashSet<_> = galaxies
        .iter()
        .permutations(2)
        .map(|v| {
            let x1 = v[0].0;
            let y1 = v[0].1;
            let x2 = v[1].0;
            let y2 = v[1].1;

            if x2 > x1 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            }
        })
        .collect();

    let res: isize = pers
        .into_iter()
        .map(|(x1, y1, x2, y2)| {
            let p = (x1, y1, x2, y2);

            let mut found = 0;
            for inx in &inxes {
                let inx = *inx as isize;
                let min = x1.min(x2);
                let max = x1.max(x2);
                if min < inx && inx < max {
                    found += 1;
                    println!("p: {:?} between empty row {inx}", p);
                }
            }

            for col in &cls {
                let col = *col as isize;
                let min = y1.min(y2);
                let max = y1.max(y2);
                if min < col && col < max {
                    found += 1;
                    println!("p: {:?} between empty col {col}", p);
                }
            }

            let d = (x1 - x2).abs() + (y1 - y2).abs();
            let d = d + found * SPACE;
            println!("p: {:?}, d: {d}", p);
            d
        })
        .sum();

    println!("result day11:  {}", res);

    Ok(())
}
