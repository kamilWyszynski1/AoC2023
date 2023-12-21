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

type Pattern<T> = Vec<Vec<T>>;

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    solve(path, Variant::A)
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    solve(path, Variant::B)
}

enum Variant {
    A,
    B,
}

fn solve<P: AsRef<Path>>(path: P, v: Variant) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(Result::ok)
        .into_iter()
        .fold(Vec::new(), |mut acc, x| {
            let x = x.unwrap();
            if x.is_empty() || acc.is_empty() {
                acc.push(Vec::new());
            }
            if !x.is_empty() {
                acc.last_mut().unwrap().push(x);
            }

            acc
        });

    match v {
        Variant::A => {
            let mut res = 0;
            for (i, pattern) in lines.iter().enumerate() {
                match search(pattern, vec_cmp_direct) {
                    Some((ref_type, inx)) => match ref_type {
                        ReclectionType::Vertical => res += inx,
                        ReclectionType::Horizontal => res += 100 * inx,
                    },
                    None => {
                        println!("not found for {i}")
                    }
                }
            }

            println!("result day13a : {res}",);
        }
        Variant::B => {
            let mut res = 0;
            for (i, pattern) in lines.iter().enumerate() {
                match search(pattern, vec_cmp) {
                    Some((ref_type, inx)) => match ref_type {
                        ReclectionType::Vertical => res += inx,
                        ReclectionType::Horizontal => res += 100 * inx,
                    },
                    None => {
                        println!("not found for {i}")
                    }
                }
            }

            println!("result day13 b: {res}",);
        }
    }

    Ok(())
}

#[derive(Debug)]
enum ReclectionType {
    Vertical,
    Horizontal,
}

fn search(pattern: &Vec<String>, vec_cmp: VecCMP) -> Option<(ReclectionType, usize)> {
    if let Some(inx) = search_horizontal(pattern, vec_cmp) {
        return Some((ReclectionType::Horizontal, inx));
    }
    if let Some(inx) = search_vertical(pattern, vec_cmp) {
        return Some((ReclectionType::Vertical, inx));
    }

    None
}

type VecCMP = fn(v1: &Vec<String>, v2: &Vec<String>) -> bool;

fn search_vertical(pattern: &Vec<String>, vec_cmp: VecCMP) -> Option<usize> {
    let transponsed = transpose(
        pattern
            .clone()
            .into_iter()
            .map(|l| l.chars().collect_vec())
            .collect_vec(),
    )
    .into_iter()
    .map(|chars| chars.into_iter().collect::<String>())
    .collect_vec();

    find(&transponsed, vec_cmp)
}

fn search_horizontal(pattern: &Vec<String>, vec_cmp: VecCMP) -> Option<usize> {
    find(pattern, vec_cmp)
}

fn find(v: &Vec<String>, vec_cmp: VecCMP) -> Option<usize> {
    let mut point = None;
    for i in 0..v.len() - 1 {
        let size = i.min(v.len() - i - 2);

        let left = v[i - size..=i].to_vec();
        let mut right = v[i + 1..=i + 1 + size].to_vec();
        right.reverse();

        println!("size: {size}");
        println!("{i} -> {},{}: {:?}", i - size, i, left);
        println!("{} -> {},{}: {:?}", i + 1, i + 1, i + 1 + size, right);

        if vec_cmp(&left, &right) && (i - size == 0 || i + 1 + size == v.len() - 1) {
            println!("FOUND");
            point = Some(i + 1);
        }
    }
    point
}

fn vec_cmp_direct(v1: &Vec<String>, v2: &Vec<String>) -> bool {
    v1 == v2
}

fn vec_cmp(v1: &Vec<String>, v2: &Vec<String>) -> bool {
    v1.iter()
        .zip(v2.iter())
        .map(|(s1, s2)| distance(s1, s2))
        .sum::<usize>()
        == 1
}

fn distance(s1: &String, s2: &String) -> usize {
    s1.chars()
        .zip(s2.chars())
        .map(|(ch1, ch2)| if ch1 == ch2 { 0 } else { 1 })
        .sum()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
