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

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut histories: Vec<Vec<isize>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect();
    dbg!(&histories);

    let mut sum = 0;
    for history in histories {
        let mut curr = history.clone();
        let mut sequences = vec![curr.clone()];

        while !curr.iter().all(|v| *v == 0) {
            dbg!(&curr);
            let mut new_sequence = vec![];
            for i in 0..curr.len() - 1 {
                new_sequence.push(curr[i + 1] - curr[i]);
            }
            dbg!(&new_sequence);
            curr = new_sequence.clone();
            sequences.push(new_sequence);
        }

        let mut seqs_iter = sequences.into_iter().rev();
        seqs_iter.next(); // skip last sequence, all zeros
        let mut last = 0;
        for seq in seqs_iter {
            let last_item = *seq.last().context("no last element")?;
            last += last_item;
        }
        sum += last;
    }

    println!("result day9 a: {sum}",);
    Ok(())
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut histories: Vec<Vec<isize>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect();
    dbg!(&histories);

    let mut sum = 0;
    for history in histories {
        let mut curr = history.clone();
        let mut sequences = vec![curr.clone()];

        while !curr.iter().all(|v| *v == 0) {
            dbg!(&curr);
            let mut new_sequence = vec![];
            for i in 0..curr.len() - 1 {
                new_sequence.push(curr[i + 1] - curr[i]);
            }
            dbg!(&new_sequence);
            curr = new_sequence.clone();
            sequences.push(new_sequence);
        }

        let mut seqs_iter = sequences.into_iter().rev();
        seqs_iter.next(); // skip last sequence, all zeros
        let mut first = 0;
        for seq in seqs_iter {
            let first_item = *seq.first().context("no last element")?;
            dbg!(first_item, first);
            first = first_item - first;
        }
        sum += first;
    }

    println!("result day9 a: {sum}",);
    Ok(())
}
