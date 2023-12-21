use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
use std::{
    any,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, Sub},
    path::Path,
};

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res = 0;

    let mut lines = reader.lines();
    let binding = lines.next().unwrap()?.replace(" ", "");
    let times: Vec<usize> = binding
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect_vec();

    let binding = lines.next().unwrap()?.replace(" ", "");
    let distances: Vec<usize> = binding
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();

    let mut ways = vec![];
    for (time, record) in times.into_iter().zip(distances.into_iter()) {
        let mut possibilities = 0;

        for i in (0..time).rev() {
            let d = (time - i) * i;
            if d > record {
                possibilities += 1
            }
        }
        if possibilities != 0 {
            ways.push(possibilities);
        }
    }

    println!("result day6 b: {:?}", ways.into_iter().product::<usize>());
    Ok(())
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res = 0;

    let mut lines = reader.lines();
    let binding = lines.next().unwrap()?;
    let times: Vec<usize> = binding
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect_vec();

    let binding = lines.next().unwrap()?;
    let distances: Vec<usize> = binding
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();

    let mut ways = vec![];
    for (time, record) in times.into_iter().zip(distances.into_iter()) {
        let mut possibilities = 0;

        for i in (0..time).rev() {
            let d = (time - i) * i;
            println!("i: {i}, d: {d}");
            if d > record {
                possibilities += 1
            }
        }
        if possibilities != 0 {
            ways.push(possibilities);
        }
    }

    println!("result day6 a: {:?}", ways.into_iter().product::<usize>());
    Ok(())
}
