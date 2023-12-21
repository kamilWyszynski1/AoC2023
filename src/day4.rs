use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Context;

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line?);
    }

    let mut copies: HashMap<usize, usize> = HashMap::new();
    for i in 1..=lines.len() {
        copies.insert(i, 1);
    }

    for (i, line) in lines.into_iter().enumerate() {
        let (_, rest) = line.split_once(": ").context("could not split")?;

        let (winning, ours) = rest.split_once("|").context("could not split numbers")?;

        let winning_numbers: Vec<isize> = winning
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect();
        let ours_numbers: HashSet<isize> = ours
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect();

        let a: isize = ours_numbers
            .into_iter()
            .map(|num| winning_numbers.contains(&num) as isize)
            .sum();

        let a = a as usize;
        let copy = copies.get(&(i + 1)).unwrap();
        for _ in 0..*copy {
            for j in i + 2..i + 2 + a {
                copies.entry(j).and_modify(|v| *v += 1);
            }
        }
    }

    println!("result day4 b: {}", copies.values().sum::<usize>());

    Ok(())
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res = 0;

    for line in reader.lines() {
        let line = line?;

        let (_, rest) = line.split_once(": ").context("could not split")?;

        let (winning, ours) = rest.split_once("|").context("could not split numbers")?;

        let winning_numbers: Vec<isize> = winning
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect();
        dbg!(ours);
        let ours_numbers: HashSet<isize> = ours
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<isize>().unwrap())
            .collect();

        let a: isize = ours_numbers
            .into_iter()
            .map(|num| winning_numbers.contains(&num) as isize)
            .sum();
        dbg!(a);

        if a != 0 {
            res += 2_isize.pow(a as u32 - 1);
        }
    }

    println!("result day4 a: {res}");
    Ok(())
}
